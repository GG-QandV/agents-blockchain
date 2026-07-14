# Live-проверка M7a-Sui против testnet (критический путь, без хвостов)

Цель: доказать, что наша подпись/адрес/перевод принимаются РЕАЛЬНЫМ Sui, а не только моками.
Пока эти 3 теста не зелёные на testnet — коннектор НЕ считается готовым.

## 0. Инструменты
- Sui CLI: https://docs.sui.io/references/cli (для эталонов keytool и сборки tx_bytes)
- `sui client faucet` — testnet USDC; НЕ пополнять SUI-токеном (чтобы доказать gasless)
- Два разных testnet RPC (напр. публичный + свой node / провайдер)

## 1. Окружение
```
export SUI_RPC1=https://<testnet-rpc-1>
export SUI_RPC2=https://<testnet-rpc-2>
# dev-ключ детерминирован SoftVault ([3;32]); адрес печатает sui-smoke при старте
export SUI_BUILD_METHOD=<метод сборки gasless-перевода из https://docs.sui.io/sui-api-ref>
export SUI_RECIPIENT=0x<32B hex получателя>
export SUI_COIN_TYPE=0x<pkg>::usdc::USDC     # coin type testnet USDC
```

## 2. Тест digest (эталон = sui keytool)
```
TX=$(sui client transfer --to <ADDR> --amount 100000 --coin-type $SUI_USDC_TYPE --serialize-unsigned-transaction)
sui keytool sign --address <ADDR> --data $TX   # взять "Digest to sign"
```
Прогнать те же TX-байты через `intent::signing_digest`, сравнить с "Digest to sign" — побайтово.
Правило Sui: digest = Blake2b-256( [0,0,0] ‖ BCS(tx_data) ).
Источник: https://docs.sui.io/concepts/cryptography/transaction-auth/intent-signing

## 3. Тест адреса
`sui keytool import` ключа SUI_TEST_KEY → адрес; сравнить с `sui_address_from_pubkey(0x02, pubkey)`.
Правило: address = Blake2b-256( flag ‖ pubkey ), flag Secp256r1 = 0x02.
Источник: https://docs.sui.io/operators/exchange-integration

## 4. Тест реального перевода
Реализовать RpcClient поверх Sui JSON-RPC:
- pick_coin: `suix_getCoins` (owner, coinType) → выбрать объект ≥ amount
- dry_run: `sui_dryRunTransactionBlock`
- execute: `sui_executeTransactionBlock` (tx_bytes + [serialized_sig], reqType WaitForLocalExecution)
- status: `sui_getTransactionBlock` (digest, {showEffects:true}) → checkpoint/status
Источник методов: https://docs.sui.io/sui-api-ref
ОЖИДАНИЕ: execute → Real → status → Settled; баланс USDC получателя +0.1; SUI-баланс отправителя без изменений.

## 5. Гейт готовности коннектора
□ digest_matches_sui_keytool зелёный
□ address_matches_sui_keytool зелёный
□ real_gasless_usdc_transfer_settles зелёный + SUI-баланс отправителя не тронут
Только после этого — интеграция коннектора в mu-runtime и этап 2 (механика).

## 5. Запуск harness
```
cargo run -p sui-smoke
```
Критерии: execute → ACCEPTED digest → status poll → Settled{checkpoint};
Unknown-исход НЕ повторять вручную (инвариант RISK-M7-1) — только status/reconcile.
