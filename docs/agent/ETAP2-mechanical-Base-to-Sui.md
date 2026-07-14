# Этап 2 (механический): миграция Base→Sui вне M7a — инструкция агенту

Предусловие: этап 1 закрыт — M7a-sui в mu-connect (sui.rs, sui_jsonrpc.rs) + sui-smoke,
101 тест зелёный, live-testnet по sui-smoke/README-LIVE.md пройден (3 эталонных сверки + перевод).

## Правила (нарушение = откат PR)
1. Инварианты не трогать: Unknown≠Failed, WalWritten, линейный Reservation, default=Keep в reconcile,
   Ω только через reissue, checked-арифметика Amount, log_head-связка. Каждый закрыт одноимённым тестом.
2. Один источник правды на правило: адресная канонизация — только mu-policy/mu-common; никаких локальных парсеров.
3. Никаких float; Cargo.lock не обновлять без прогона всех тестов (пины под Rust 1.75+).
4. PR в зоне RISK-комментария мержится только с зелёным соответствующим тестом.

## Шаги (порядок обязателен — по зависимостям)

### Ш1. mu-common: адрес 20B→32B (~70 LOC)
CanonAddress: [u8;20]→[u8;32]; hex-длина 40→64; EIP-55-логика удаляется из хука.
Sui-формат: 0x + 64 hex, без чексуммы регистра.
Источник: https://docs.sui.io/concepts/cryptography/transaction-auth/keys-addresses

### Ш2. mu-policy (~80 LOC)
eip55.rs → sui_addr.rs: canon_address_checked = длина/hex; W-ADR-01 переформулировать
(«адрес без чексуммы» → предупреждение о ручной сверке identicon — чексуммы в Sui нет).
Удалить sha3/keccak из зависимостей. E-ADR-03: chain_id → network ("mainnet"|"testnet").
Тест-векторы: адреса из sui keytool (эталон), не выдуманные.

### Ш3. M1 mu-core (~40 LOC)
format.rs: поле адреса в Delta-слоте 20→32B; FORMAT_VERSION=2; decode v1 → VersionUnsupported
(миграция старых μ = reissue, не тихий апгрейд). Тест bit_flip прогнать заново (позиции сместились).

### Ш4. M2/M3 в mu-runtime/policy.rs (~25 LOC) — УПРОЩЕНИЕ
omega_check: убрать gas_estimate, total = amount. Удалить RISK-M7-6-логику запаса.
needs_human(amount) — по сумме. Тесты границ threshold пересчитать без gas.

### Ш5. M6 pipeline (~50 LOC)
Заменить вызовы crypto-коннектора на SuiConnector (или общий trait-адаптер);
quote → всегда Amount::ZERO (gasless); recipient 32B из intent; PayConfirm.gas_est удалить (M9 тоже).

### Ш6. M5 reconcile (~90 LOC)
NonceState → SuiTxState: ConsumedByOther ≈ «coin-объект потрачен другой tx (версия ушла) И нашего
digest нет с ≥1 checkpoint у ОБЕИХ нод»; NotReached ≈ версия объекта нетронута. Свойства теста
exhaustive_table_properties сохранить дословно (Unreachable → никогда ToFailed).
Источник модели объектов: https://docs.sui.io/concepts/object-ownership

### Ш7. M8 gate (~8 LOC) + M9 (~10 LOC)
wire: MAX recipient 64→80 (0x+64hex). M9: убрать gas из диалога; render_recipient_line без изменений.

### Ш8. Composer (core/cli/tauri) (~60 LOC)
proposal: адрес 20→32B (+версия формата proposal=2, старый → Malformed);
CLI/UI: CHAIN-константу → NETWORK; identicon работает от hex без правок.

### Ш9. Fixtures + сквозные тесты (~430 LOC правок)
Все адресные литералы 0x…(40) → 0x…(64); мок RPC — Sui-варианты; fixtures: whitelist из
testnet-адресов; AGENT-smoke: DoD-пункт «SIMULATED-цикл» без изменений.

### Ш10. Спеки/доки
SPEC M7a: секции RISK-M7-2/6 заменить формулировками из комментариев mu-connect/src/sui.rs
(они и есть новая нормативка). SPEC M4: добавить «кошелёк = P-256 (flag 0x02), оговорка RAM снята».
Архитектура v4: L2/L5 = Sui gasless (protocol-level), USDC coin-type — константа сборки.
User guide: блок ETH удалить; «сеть при загрузке = Sui».

## Definition of Done
□ cargo test --workspace: все зелёные (ожидание ~101±, ноль skipped из-за миграции)
□ grep -r "keccak\|EIP-55\|eip55" по src — пусто □ grep gas_estimate вне mu-connect — пусто
□ fixtures→daemon boot→socket→SIMULATED цикл на 32B-адресах
□ live: sui-smoke Settled на testnet повторён после Ш5 через полный конвейер демона
