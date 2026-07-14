# Инструкция агенту: smoke-тестирование μ-демона на dev-фикстурах

## Источник
`mu-fixtures.tar.gz` → крейт `mu-fixtures/` — добавить в workspace members рабочего репо
(рядом с mu-daemon). DEV-ONLY: SoftVault, детерминированные ключи ([1;32]/[2;32]/[3;32], агент seed [42;32]).
В прод-сборку не включать.

## 1. Генерация артефактов
```
export MU_HOME=/tmp/mu-smoke && rm -rf $MU_HOME
cargo run -p mu-fixtures
```
Создаёт: `mu.bin` (лимит 500 USDC, порог 100, whitelist: 0x5aAe…eAed), `log.mulog`
(валидная цепь), `agent.key` (ed25519 seed). Печатает mu_pubkey и agent_pubkey.

## 2. Boot-smoke демона
```
MU_HOME=/tmp/mu-smoke ./target/debug/mu-daemon   # конфиг: SoftVault, mu_pubkey из вывода фикстур
```
Ожидание: старт проходит (verify → verify_chain → verify_against_log → reconcile → listen),
socket-файл создан.

## 3. Rollback-smoke (DoD-пункт из AGENT-mu-daemon)
```
rm -rf $MU_HOME && cargo run -p mu-fixtures -- rollback-attack
MU_HOME=/tmp/mu-smoke ./target/debug/mu-daemon
```
Ожидание: демон НЕ стартует, причина LogHeadMismatch (подпись μ при этом валидна — атака
ловится именно связкой с логом). Проверено тестом `rollback_fixture_fails_boot` в крейте.

## 4. Socket-smoke (intent → SIMULATED)
Кадр для gate: payload из mu-gate/src/wire.rs (v=1, recipient="0x5aae…" hex-строка,
amount u128 BE, chain 8453, agent_id="agent-1", nonce=1, ts=now) ‖ ed25519-подпись payload
ключом из agent.key. Референс сборки кадра — функция `frame_signed` в mu-gate/src/gate.rs (тесты).
Отправить в socket (len:u32 LE ‖ кадр). Ожидание: в log.mulog появляются Pending → Failed +
Simulated (bank_stub), window_sum не изменился.

## 5. Автотесты фикстур (входят в крейт)
```
cargo test -p mu-fixtures      # normal_fixture_boots + rollback_fixture_fails_boot
```

## Definition of Done
□ п.2 стартует □ п.3 отказывает с LogHeadMismatch □ п.4 даёт Simulated-цикл в логе
□ cargo test -p mu-fixtures — 2 passed.
