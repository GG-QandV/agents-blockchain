# μ-agentic MVP v3 — модульная декомпозиция

## 1. Карта модулей

```
μ-system
├── M1  mu-core        μ-объект: структура, сериализация, подпись, checksum
├── M2  mu-omega       Ω: статичный фильтр возможностей
├── M3  mu-delta       Δ: правила, check-and-reserve, скользящее окно
├── M4  mu-vault       ключи: enclave-обёртка, выдача на подпись
├── M5  mu-log         WAL + hash-chain, сверка при старте
├── M6  mu-runtime     оркестратор: очередь, конвейер Ω→Δ→WAL→exec
├── M7  mu-connect     интерфейс Connector + реализации
│   ├── M7a crypto     живой (USDC/L2)
│   ├── M7b bank_stub  симуляция
│   └── M7c card_stub  симуляция
├── M8  mu-gate        вход агента: socket, подпись, allowlist
└── M9  mu-human       биометрия, TTL, просмотр журнала
```

## 2. Модули: интерфейсы и будущий код

| M                 | Публичный интерфейс (сигнатуры)                                                                                   | Внутренности / код                                                                                                                                     | Зависит от  |
| ----------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------- |
| **M1 mu-core**    | `load(path)→Mu` `save(Mu)` `verify(Mu)→bool` `reissue(Mu, newOmega, ownerKey)→Mu`                                 | Структуры 5 слотов; CBOR encode/decode; COSE-подпись через M4; checksum; атомарная запись (tmp+fsync+rename). ~400 строк                               | M4          |
| **M2 mu-omega**   | `check(intent, omega)→Allow\|Deny(reason)`                                                                        | Чистая функция без состояния: операция ∈ operations, коннектор ∈ connectors, amount+gas ≤ ceiling. ~50 строк                                           | —           |
| **M3 mu-delta**   | `check_and_reserve(intent, delta, log)→Reservation\|Deny` `commit(r)` `rollback(r)` `needs_human(intent)→bool`    | Сумма SETTLED+PENDING за 24ч из M5; whitelist адресов; резерв = PENDING-запись. ~150 строк                                                             | M5          |
| **M4 mu-vault**   | `sign_mu(bytes)→sig` `sign_tx(bytes)→sig` `wrap(key)` / `unwrap()→zeroizing<key>`                                 | Платформенные ветки (Enclave/StrongBox/TPM); P-256 в железе; secp256k1 unwrap→подпись→zeroize (обнуление памяти). Самый платформозависимый, ~600 строк | ОС API      |
| **M5 mu-log**     | `append(Entry)→hash` `pending()→[Entry]` `window_sum(24h)→amount` `verify_chain()→bool` `reconcile(chain_client)` | Entry{prev_hash, ts, intent, decision, status, tx_hash}; подпись каждой записи через M4; сверка PENDING с цепью. ~250 строк                            | M4, M7      |
| **M6 mu-runtime** | `submit(intent)→ticket` `status(ticket)`                                                                          | Однопоточная очередь (mpsc-канал); конвейер; таймауты; старт = M5.reconcile до открытия M8. ~300 строк                                                 | все         |
| **M7 mu-connect** | trait: `quote(intent)→fee` `execute(intent, signer)→TxRef` `status(TxRef)→Pending\|Settled\|Failed`               | M7a: ERC-20 transfer, EIP-1559, nonce-менеджмент, 2 RPC со сверкой (~400 стр). M7b/c: возврат SIMULATED+фейковый TxRef (~40 стр каждая)                | M4 (signer) |
| **M8 mu-gate**    | `listen(socket_path, allowlist)` → intent'ы в M6                                                                  | Unix socket/named pipe; проверка Ed25519-подписи запроса; rate-limit на агента. ~200 строк                                                             | M6          |
| **M9 mu-human**   | `confirm(intent, ttl)→Approved\|Denied\|Timeout` `view_log(filter)`                                               | Платформенный биометрический диалог; таймер TTL; read-only рендер M5. ~250 строк                                                                       | M5, ОС API  |

**Правила зависимостей:** M2, M3 — чистая логика без I/O (тестируются без окружения); секреты не покидают M4 (наружу — только подписи); M7 не видит μ-объект целиком — получает intent и signer-хэндл.

## 3. Датафлоу

### 3.1. Основной поток (оплата)

```
Agent
  │ ①JSON+подпись
  ▼
M8 gate ──②verify sig, allowlist──✗→ drop
  │ ③intent
  ▼
M6 runtime (очередь, по одному)
  │ ④
  ▼
M2 omega.check ──✗──────────────────────→ M5.append(DENIED_Ω) → ответ агенту
  │ ⑤ok
  ▼
M3 delta.check_and_reserve ──✗──────────→ M5.append(DENIED_Δ) → ответ
  │ ⑥ok, > threshold?
  ├─да→ M9.confirm(TTL 15м) ──Timeout/Deny→ M3.rollback → M5.append → ответ
  │ ⑦approved
  ▼
M5.append(PENDING)                ← WAL: до денег
  │ ⑧
  ▼
M7a crypto.execute
  │   ⑨ M4.unwrap→sign_tx→zeroize
  │   ⑩ tx → RPC₁,RPC₂ → финальность
  ▼
M5.append(SETTLED, tx_hash) + M3.commit     [или FAILED + rollback]
  │ ⑪
  ▼
M6 → M8 → Agent: {status, ticket}   — без ключей, адресов источника, деталей Δ/Ω
```

### 3.2. Рекавери-поток (старт демона)

```
start → M1.load+verify(μ) ──✗→ halt (объект повреждён)
      → M5.verify_chain ──✗→ halt
      → M5.pending() → для каждой: M7a.status(tx_hash из цепи)
             найдена в цепи → SETTLED ; нет → FAILED+rollback
      → только затем M8.listen
```

### 3.3. Поток изменения правил

```
Owner → M9(биометрия) → новый Δ → подпись owner-ключом (M4) → M1.save
Ω изменить нельзя: только M1.reissue → новый μ (новый ID, старый Log архивируется)
```

## 4. Порядок реализации (по зависимостям)

1. M1+M2+M3 с mock-M4/M5 — вся логика тестируема без железа и сети.
2. M5 (файловый) + M7b заглушка → полный конвейер в SIMULATED.
3. M4 (одна платформа) → живые подписи.
4. M7a на testnet → mainnet микро-суммы.
5. M8, M9 — обвязка.

✓ μ разделён на 9 модулей с интерфейсами, объёмом кода, тремя датафлоу и порядком сборки
