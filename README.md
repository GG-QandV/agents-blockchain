# μ-agentic MVP — модули M4, M7a, M8 (референс-реализация)

Компилируется на Rust 1.75+. Тесты: `cargo test --workspace --features mu-vault/softvault` → 34 passing.

## Что реализовано полностью
- **mu-common** — Amount (checked-арифметика без float, RISK-X-1), Clock (два времени, RISK-X-2), CanonAddress/ConnectorId (закрытые типы).
- **M4 mu-vault** — trait Vault, доменные теги (RISK-M4-5), TxSigner с Zeroizing/!Clone/!Debug (RISK-M4-1), SoftVault (полный, тестовый), owner_sign требует auth (RISK-M4-3).
- **M7a mu-connect** — CryptoConnector: classify_send с инвариантом Unknown≠Failed (RISK-M7-1), self-check calldata (RISK-M7-5), консенсус 2 RPC в status (RISK-M7-3); стабы только Simulated (RISK-M7S-1). RPC за trait → вся таксономия ошибок тестируется без сети.
- **M8 mu-gate** — hardened-парсер с bounds-checked курсором (RISK-M8-3, тест без паник на мусоре), конвейер 1-8 с ed25519, монотонный nonce переживает рестарт (RISK-M8-1), подпись↔agent_id (RISK-M8-2), единый deny{code} (RISK-M8-5).

## Границы (честно)
- Платформенные бэкенды M4 (AppleEnclave/AndroidStrongBox/TPM2) — КАРКАС с FFI-TODO: реализуются и проверяются на устройствах (device-smoke). SoftVault полнофункционален для CI.
- Сериализация raw EVM-транзакции (RLP+подпись) в M7a упрощена до детерминированного sighash для мок-тестов; транспортный слой RLP подключается вместе с реальным RpcClient (ethers/alloy).
- EIP-55 keccak-валидация в CanonAddress — хук (реализуется в mu-policy с keccak256).
- Сокет-транспорт M8 (unix socket/named pipe) — обёртка над Gate::accept; здесь реализована чистая логика конвейера, тестируемая напрямую.

## Соответствие спекам
Каждая RISK-секция из SPEC_M4_M7a_M8_v1 имеет тест с тем же идентификатором в комментарии.
