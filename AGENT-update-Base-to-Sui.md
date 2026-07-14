# Инструкция агенту: обновление существующего репо на Base→Sui

## Источники
- `mu-v2-sui.tar.gz` — эталонный workspace с готовым этапом 1 (M7a-sui, 101 тест)
- `ETAP2-mechanical-Base-to-Sui.md` — пошаговая механика этапа 2 (Ш1–Ш10, источники, DoD)
- `sui-smoke/README-LIVE.md` (внутри архива) — live-проверка против testnet

## Порядок

### Фаза A — влить этап 1 (готовый код)
1. Распаковать эталон рядом с рабочим репо. Перенести **как есть**:
```
mu-connect/src/sui.rs            # коннектор (dry-run self-check, classify Unknown≠Failed)
mu-connect/src/sui_jsonrpc.rs    # транспорт + base58
mu-connect/tests/sui_risks.rs    # 12 тестов рисков
sui-smoke/                       # live-harness + README-LIVE
```
2. Перенести правки M4 (сверить diff'ом, не переписывать руками):
   `mu-vault/src/signer.rs` — добавлен TxSignerP256; `lib.rs` — метод трейта tx_signer_p256
   (default KeyMissing); `backend.rs` — реализация в SoftVault. TxSigner (k256) НЕ удалять
   до завершения Ш5 этапа 2 — его использует старый EVM-путь и pipeline.
3. Синхронизировать зависимости mu-connect: + blake2 =0.10.6, base64 =0.21.7,
   dev: p256; ed25519-dalek из mu-connect убрать (остаётся только в mu-gate).
4. Добавить sui-smoke в workspace members.
5. Гейт фазы A: `cargo test --workspace --features mu-vault/softvault` — было N тестов,
   стало N+14, ноль красных. Прежние EVM-тесты обязаны остаться зелёными (сосуществование).

### Фаза B — live-testnet (блокирующая, до любой механики)
По `sui-smoke/README-LIVE.md`: сверка digest и адреса с эталоном `sui keytool` (§2–3),
затем реальный перевод testnet-USDC (§4–5). SUI_BUILD_METHOD взять из
https://docs.sui.io/sui-api-ref (метод сборки gasless Address-Balances перевода —
сверить по документации, в коде это конфиг-строка). 
Критерий: Settled{checkpoint} при нулевом SUI-балансе отправителя (доказательство gasless).
Любое расхождение digest/адреса с keytool → стоп, чинить signing_digest/sui_address_from_pubkey,
НЕ обходить подгонкой констант.

### Фаза C — механика по ETAP2 (Ш1–Ш10 строго по порядку)
После каждого шага — полный тест-прогон; коммит на шаг. По завершении Ш5 удалить
старый EVM-путь (crypto.rs, TxSigner/k256, тесты EVM) одним коммитом — grep-гейты DoD
из ETAP2 подтверждают чистоту.

## Запрещено
Менять инварианты (список в ETAP2 §Правила); редактировать sui.rs/sui_jsonrpc.rs при вливании
(фаза A = перенос, не рефакторинг); мержить фазу C без зелёной фазы B.

## Definition of Done (сквозной)
□ Фаза A: +14 тестов, старые зелёные □ Фаза B: digest==keytool, адрес==keytool,
Settled на testnet без SUI-баланса □ Фаза C: DoD ETAP2 (grep keccak/gas_estimate пусто,
fixtures→daemon→SIMULATED на 32B-адресах, live повторён через полный конвейер)
