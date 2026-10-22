# μ Gate KSG — чеклист плана

Ветка `keysingate`. Обновлено 2026-10-22. Команда проверки:
`cargo test --workspace --features mu-vault/softvault`.

## P0 — блокеры production

| # | Пункт | Статус | Коммит |
|---|---|---|---|
| P0-1 | Конфиг демона из `MU_HOME` (ключи 0600, `agents.toml`, `policy.toml`), без вкомпилированных ключей | ✅ | `b63c71e` |
| P0-2 | Wire v2: интент несёт `resource`, поле доходит до `RtIntent` | ✅ | `95f6e1b` |
| P0-3 | Коннектор из `MU_CONNECTOR` — в интенте и в Ω | ✅ | `95f6e1b` |
| P0-4 | Ответ `0x20 ‖ len:u16 LE ‖ X-PAYMENT`; клиенты Py/TS читают кадр | ✅ | `1453442` |
| P0-5 | `PendingAuth` в WAL до платежа, nonce из `OsRng`, x402 без nonce не подписывает, reconcile на старте (journal format 2) | ✅ | `168c47d` |
| P0-6 | Подтверждение владельцем через stdin, owner-подпись из vault, `Vault::owner_pubkey()` | ✅ | `47ac16c` |
| P0-7 | Policy-endpoint для Composer: `<MU_HOME>/mu-policy.sock`, validate → TOCTOU → дифф → подтверждение → `DeltaChanged` | ✅ | `82f371c` |
| P0-8 | Статус x402 из `authorizationState` по двум узлам (`MU_X402_RPC_1/2`), расхождение → `Unknown` | ✅ | `d7106da` |
| P0-9 | Тест self-check на чужого получателя (прежний сверял интент сам с собой) | ✅ | `43bc8d2` |
| P0-10 | Owner-proof хеш привязан к connector и resource | ✅ | `43bc8d2` |

Тесты: **165 passed, 0 failed**.

Документация: `mu-daemon/README.md` (env, файлы, протоколы сокетов), `CHANGELOG-ksg.md`
(journal format 2, policy-endpoint, хеш, тест), `README.md` (счётчик тестов) — `454d0e1`.

## P1 — следующие пункты

| # | Пункт | Статус |
|---|---|---|
| P1-… | Список P1 в переписке утерян (срезан компакцией), в репозитории его нет | ⏳ нужен текст списка |
| — | Vault: сейчас `softvault` — ключи в открытом hex, `hw_backed: false`; P1 требует backend, снимающий этот блокер. Без формулировки не начинаю: правило «не добавлять функции вне списка» | ⏳ зависит от P1-списка |

## Открытые внешние блокеры (не код)

| Что | Состояние |
|---|---|
| Сквозной тест на Base Sepolia | не выполнялся: нужен профинансированный кошелёк + два RPC + живой issuer |
| `OPENCODE_API_KEY` (opencode zen через шлюз) | просрочен (`api_key_expired`); ключ openmodel.ai заменён на рабочий |
| `*-free` модели через шлюз | запрещены провайдером (`FreeTierError`: только изнутри OpenCode) |

## Git

| Репозиторий | Ветка | Впереди remote | Незакоммичено |
|---|---|---|---|
| `agents_blockchain` | `keysingate` | 12 | 0 |
| `keysingate` | `main` | 1 | 2 неотслеживаемых (`.claude/skills/gitnexus/`, `.run-task-20261018.sh`) |
| `~/.config/opencode` | `main` | 1 | 0 |
| `Project_mnemostroma` | `main` | 4 | 86 неотслеживаемых (нужно решение, что добавлять) |

Пуш не выполнялся ни по одному репозиторию — только по явной команде.
