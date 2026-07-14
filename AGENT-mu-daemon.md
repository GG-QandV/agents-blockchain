# Инструкция агенту: сборка μ-демона в рабочее репо

## Источник
`mu-v1.tar.gz` → workspace `mu/` с крейтами: mu-common (X), mu-core (M1), mu-vault (M4), mu-log (M5), mu-runtime (M6, внутри — Ω/Δ = M2/M3 в policy.rs), mu-connect (M7a + стабы M7b/c), mu-gate (M8), mu-human (M9). Cargo.lock включён — **не удалять**: версии крипто-крейтов пинованы под Rust 1.75+.

## Шаги

1. **Базовая проверка**:
```
tar -xzf mu-v1.tar.gz && cd mu
cargo test --workspace --features mu-vault/softvault    # ожидание: 85 passed
```
2. **Правила фич**: `softvault` — ТОЛЬКО тесты/CI. В release-профиле демона добавить защёлку:
```rust
#[cfg(all(not(debug_assertions), feature = "softvault"))]
compile_error!("softvault запрещён в release");
```
3. **Собрать бинарь демона** (нового крейта `mu-daemon` в архиве нет — создать): main = boot-протокол из спеки M6 §4, строго в порядке:
```
M1: Mu::load + verify(mu_pubkey) + verify_against_log(log.last_hash())  → ✗ = exit(2)
M5: Log::open + verify_chain                                            → ✗ = exit(3)
M5: pending() → M7a.status → mu_log::reconcile::resolve (Keep по умолчанию)
M8: восстановить nonce (gate.restore_nonce из NonceSnapshot/записей лога)
только затем: слушать unix socket → кадры → Gate::accept → Runtime::process
```
Транспорт M8 — тонкая обёртка: unix socket 0600, кадр len:u32 LE, тело → `Gate::accept(frame, clock.now_unix())`.

4. **Доделки до прода** (каркасы уже размечены в коде):
- M4: FFI-бэкенды Enclave/StrongBox/TPM (`mu-vault/src/backend.rs`, TODO-заглушки open()); проверяются device-smoke, не в CI.
- M7a: реальный `RpcClient` (JSON-RPC + RLP-сериализация tx, alloy/ethers) вместо мок-trait; таксономию ошибок НЕ трогать.
- Вынос M2/M3 из `mu-runtime/src/policy.rs` в крейты — механический, интерфейсы готовы.
- policy-endpoint для Composer: принять кадр → `composer_core::handle_propose`, approve-замыкание = M9.confirm_delta → M4.owner_sign → M1.apply_delta + save → M5.append(DeltaChanged).

## Запрещено менять (RISK-инварианты, каждый закрыт одноимённым тестом)
- `ConnErr` без варианта Failed в execute (Unknown ≠ Failed); catch-all → ReconcilePending.
- `WalWritten`: execute только после durable Pending (fsync).
- `TxSigner`: !Clone/!Debug, конструктор pub(crate).
- reconcile: default = Keep; ToFailed только Reverted×2 или nonce ConsumedByOther.
- `Amount`: только checked_*, никаких float в денежном пути.
- Ω меняется только `reissue`; Δ — только `apply_delta` с подписью владельца.
Правило PR: изменение в зоне RISK-комментария мержится только с зелёным соответствующим тестом.

## Definition of Done
□ 85 тестов зелёные □ release без softvault (compile_error проверен) □ демон стартует по boot-протоколу и отказывает в старте на битом μ/логе □ смоук: intent через сокет → SIMULATED-цикл на bank_stub виден в логе □ rollback-тест: подмена mu.bin старой копией → демон не стартует (LogHeadMismatch).
