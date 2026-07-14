# Архитектура μ-agentic MVP v4 — Sui (gasless)

## 1. Назначение

Децентрализованный агент платежей. Агент инициирует USDC-платежи на Sui по правилам владельца (Ω/Δ), никогда не имея доступа к ключу кошелька. Все решения владельца и результаты исполнения логируются в hash-chain (M5).

## 2. Модульная карта

```
μ-system (Sui, gasless)
├── M1  mu-core        μ-объект: структура, TLV-сериализация, подпись, checksum
├── M2  mu-omega       Ω: static filter (connectors, max_ceiling)
├── M3  mu-delta       Δ: правила, whitelist, скользящее окно 24ч
├── M4  mu-vault       ключи P-256 (Enclave/StrongBox/TPM/SoftVault), TxSigner, TxSignerP256
├── M5  mu-log         WAL + hash-chain (Sui-семантика: object-version замість nonce)
├── M6  mu-runtime     оркестратор: очередь, конвейер Ω→Δ→WAL→exec (gasless)
├── M7  mu-connect     SuiConnector (gasless USDC, P-256, Blake2b, dry-run self-check)
├── M8  mu-gate        вход: unix socket 0600, ed25519-подпись, allowlist, rate-limit
└── M9  mu-human       подпись владельца (owner_sign), TTL, без gas_est
```

> M2/M3 тимчасово вбудовані в `mu-runtime/src/policy.rs` — виносяться в окремі крейти за потребою.

## 3. Датафлоу (Sui gasless)

```
Agent
  │ ① JSON+ed25519-подпись (unix socket)
  ▼
M8 gate ── ② verify sig, allowlist, nonce, ts ──✗→ DenyCode
  │ ③ VerifiedIntent {recipient, amount, chain_id, agent_id, nonce}
  ▼
M6 runtime (очередь, по одному)
  │ ④ Ω-check (connector in allowlist, amount ≤ ceiling)    ← без gas_estimate
  │ ⑤ Δ-check (recipient in whitelist, окно 24ч не превышен)
  │ ⑥ human? approved? (owner_sign, P-256, DomainTag::MuHuman)
  │ ⑦ WAL: Pending{intent_hash, amount_total} → fsync
  │ ⑧ execute (Sui gasless: server-side build → dry-run → sign → send)
  │ ⑨ Settled/Failed/ReconcilePending
  ▼
M5 log (hash-chain, fsync-d до Ok)
```

## 4. Ключові відмінності від EVM-версії (Base)

| Аспект | Base (EVM) | Sui (gasless) |
|--------|-----------|---------------|
| Адреса | 20 байт (Ethereum) | 32 байти (Sui, blake2b256) |
| Формат | EIP-55 checksum | hex без чексумми |
| Підпис транзакції | k256/secp256k1 + recovery | P-256/secp256r1, RFC 6979 (flag 0x02) |
| Хеш | SHA-256 (EVM sighash) | Blake2b-256(intent ‖ tx_bytes) |
| Gas | model gas: amt + gas_estimate | protocol-level gasless (total = amount) |
| Digest | tx_hash (32B hex) | base58(blake2b256) |
| Nonce | account nonce (on-chain) | digest-детермінований (ідемпотентність) |
| Self-check | decode calldata → recipient+amount | dry-run обох нод → звірка recipient+amount |
| Reconcile | nonce-state (ConsumedByOther=nonce з'їдено іншою tx) | object-version (ConsumedByOther=версія coin зросла) |
| USDC contract | константа per chain_id | coin type-tag (конфіг, RISK-M7-5) |

## 5. FORMAT_VERSION

- v1: 20B адреси, EIP-55 (Base/ETH era)
- v2: 32B адреси, без чексумми (Sui era)
- Міграція: старий μ відкидається `VersionUnsupported`; reissue для конвертації

## 6. Специфіка Sui

### 6.1 Дайджест підпису
```
digest = Blake2b-256([0, 0, 0] ‖ tx_bytes)
         │ intent (scope=0, version=0, app_id=0)
```
Детермінований від tx_bytes: один digest = одна tx навіть при повторній відправці (ідемпотентність).

### 6.2 Адреса гаманця
```
address = Blake2b-256(0x02 ‖ pubkey_compressed)
                      │ flag Secp256r1 (P-256)
```
Еталон звірки: `sui keytool import` + `sui keytool list`.

### 6.3 Gasless
Sui не вимагає SUI-токенів для gas, якщо тип активу (USDC) має спонсора або використовує Address-Balances модель. `Fee::gas_estimate = Amount::ZERO` завжди.

### 6.4 Self-check (RISK-M7-5)
Перед підписом обидва RPC независимо збирають tx і dry-run. Якщо recipient/amount не збігаються з intent — відмова до підпису (`Rejected::InvalidRecipient`).

### 6.5 Консенсус (RISK-M7-3)
Settled тільки при згоді обох нод про checkpoint + success. Для reconcile використовується стан coin-об'єкта (версія) замість EVM nonce.

## 7. Склад модулів (поточна реалізація)

| Модуль | Статус | LOC (прибл.) |
|--------|--------|-------------|
| mu-common | ✅ Complete | 150 |
| mu-core (M1) | ✅ Complete | 220 |
| mu-vault (M4) | ✅ Complete (SoftVault + каркаси) | 160 |
| mu-log (M5) | ✅ Complete | 250 |
| mu-runtime (M6) | ✅ Complete | 220 |
| mu-connect/sui (M7a) | ✅ Сonneсtor готовий, live-testnet pending | 200 |
| mu-connect/stub (M7b/c) | ✅ Stubs для тестів | 40 |
| mu-gate (M8) | ✅ Complete | 230 |
| mu-human (M9) | ✅ Complete | 200 |
| mu-policy | ✅ Complete | 270 |
| mu-daemon | ✅ Binary (boot+serve) | 250 |
| mu-fixtures | ✅ Dev fixtures | 100 |
| composer-core | ✅ Core logic | 350 |
| composer-cli | ✅ CLI binary | 140 |
| composer-tauri | ✅ GUI binary | 150 |
| sui-smoke | ✅ Live test harness | 100 |
| **Total** | | **~2,830** |

## 8. Live-testnet (Фаза B — pending)

Для завершення перевірки коннектора на реальному testnet потрібно:
1. Поповнити гаманець `0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb` testnet USDC
2. Налаштувати `SUI_RPC1`, `SUI_RPC2`, `SUI_COIN_TYPE`, `SUI_BUILD_METHOD`
3. Запустити `cargo run -p sui-smoke`
4. Очікується: `execute → ACCEPTED → status → Settled{checkpoint}`

**Адреса пройдена** ✅ (збігається з `sui keytool`).
**Digest/Transfer** — потребують testnet USDC на гаманці.

## 9. RISK-інваріанти (незмінні при міграції)

| Інваріант | Суть | Покриття |
|-----------|------|----------|
| Unknown ≠ Failed | execute не може повернути Failed (тільки Rejected/Unknown) | sui_risks.rs |
| WalWritten | execute тільки після durable Pending (fsync) | pipeline_risks.rs |
| TxSigner | !Clone/!Debug, конструктор pub(crate) | signer.rs (обидва TxSigner) |
| reconcile default=Keep | ToFailed тільки Reverted×2 або ConsumedByOther×2 | store_risks.rs |
| Amount | Тільки checked_* (ніяких float) | amount.rs |
| Ω | Тільки через reissue | core_risks.rs |
| Δ | Тільки apply_delta з підписом власника | core_risks.rs |
