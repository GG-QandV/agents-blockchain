# μ-agentic MVP — модули M4–M9 + Composer часть 1 (референс-реализация)

Компилируется на Rust 1.75+. Тесты: `cargo test --workspace --features mu-vault/softvault` → 85 passing.

## M1 mu-core (μ-объект)
- Приватные слоты (нет &mut к Ω — RISK-M1-1 на уровне типов) + побайтная сверка Ω в apply_delta.
- apply_delta требует валидную подпись владельца (домен mu.delta.v1); reissue = новый id, version+1.
- **Rollback-атака ловится** (RISK-M1-2): сквозной тест с M5 — старый валидно подписанный μ с большим лимитом отвергается по несовпадению log_head с хвостом цепи.
- Атомарная запись tmp+fsync+rename+fsync(dir); mu.prev — диагностика, авто-отката нет (RISK-M1-3).
- Строгий TLV-парсер: bit-flip в любой позиции файла детектируется (decode или подпись); лишний байт = ошибка (RISK-M1-4).
- NB: формат TLV (семейство mu-wire), не COSE/CBOR из спеки — мост механический, как у proposal. CLI: `cargo run -p composer-cli -- show`.

## Что реализовано полностью
- **mu-common** — Amount (checked-арифметика без float, RISK-X-1), Clock (два времени, RISK-X-2), CanonAddress/ConnectorId (закрытые типы).
- **M4 mu-vault** — trait Vault, доменные теги (RISK-M4-5), TxSigner с Zeroizing/!Clone/!Debug (RISK-M4-1), SoftVault (полный, тестовый), owner_sign требует auth (RISK-M4-3).
- **M7a mu-connect** — CryptoConnector: classify_send с инвариантом Unknown≠Failed (RISK-M7-1), self-check calldata (RISK-M7-5), консенсус 2 RPC в status (RISK-M7-3); стабы только Simulated (RISK-M7S-1). RPC за trait → вся таксономия ошибок тестируется без сети.
- **M8 mu-gate** — hardened-парсер с bounds-checked курсором (RISK-M8-3, тест без паник на мусоре), конвейер 1-8 с ed25519, монотонный nonce переживает рестарт (RISK-M8-1), подпись↔agent_id (RISK-M8-2), единый deny{code} (RISK-M8-5).

- **M5 mu-log** — durable append (fsync до Ok, RISK-M5-1), hash-chain + verify (RISK-M5-2), обрезка битого хвоста с маркером TailTruncated (RISK-M5-5), window_sum с исключением Simulated/Failed (RISK-M5-4), reconcile: чистая табличная résolution с исчерпывающим перебором 48 комбинаций и свойством «Unreachable никогда не даёт ToFailed» (RISK-M5-3).
- **M6 mu-runtime** — конвейер Ω→Δ→human→WAL→execute; typestate WalWritten: execute недостижим без durable Pending (RISK-M6-1); Unknown → ReconcilePending с удержанием резерва (RISK-M6-2/5, тест unknown_keeps_reserve_blocks_next); верификация auth_proof в конвейере (RISK-M9-2); catch-all non_exhaustive ошибок → консервативная ветка. Ω/Δ встроены в policy.rs (по карте — M2/M3, интерфейсы совпадают со спеками, выносятся в отдельные крейты без изменений).
- **M9 mu-human** — confirm_payment: Approved = подпись owner-ключом над intent_hash (RISK-M9-2, garbage proof отклоняется), привязка proof↔intent (RISK-M9-1), TTL по монотонным часам (RISK-M9-5), label всегда с усечённым адресом (RISK-M9-3), отказ биометрии → Denied (сквозной тест broken_biometry_cannot_settle). Платформенные диалоги — за trait Presenter (реализуются на устройствах).

- **mu-policy** — полные таблицы валидации E-*/W-* (§7 спеки Composer), EIP-55 с keccak256 и официальными векторами (закрывает хук mu-common), порядконезависимый delta_hash.
- **composer-core** — C3 черновики (атомарная запись, corrupt → отброс), C4 proposal (hardened TLV, roundtrip+junk-тесты) и кадрирование транспорта, приёмный endpoint с TOCTOU/Stale (§14.3: гонка двух Composer'ов → ровно один Applied) и троян-тестом (§14.5: подмена после UI видна в diff демона).
- **composer-cli (C5)** — mu-compose: show/set-limit/set-threshold/wl-add/wl-rm/hash/propose; offline-режим (F-08), propose через unix socket.

## Границы (честно)
- Платформенные бэкенды M4 (AppleEnclave/AndroidStrongBox/TPM2) — КАРКАС с FFI-TODO: реализуются и проверяются на устройствах (device-smoke). SoftVault полнофункционален для CI.
- Сериализация raw EVM-транзакции (RLP+подпись) в M7a упрощена до детерминированного sighash для мок-тестов; транспортный слой RLP подключается вместе с реальным RpcClient (ethers/alloy).
- EIP-55 keccak-валидация в CanonAddress — хук (реализуется в mu-policy с keccak256).
- Сокет-транспорт M8 (unix socket/named pipe) — обёртка над Gate::accept; здесь реализована чистая логика конвейера, тестируемая напрямую.

- M2/M3 остаются встроенными в mu-runtime/policy.rs (интерфейсы по спекам; вынос в крейты — механический).
- M5: рабочая копия записей в памяти, один сегмент (сегментация 8 МиБ и кэш границ — фаза 2 по RISK-M5-6); verify подписи записей отделён от verify_chain (нужен pubkey μ извне).
- M6: очередь как &mut self (сериализация); mpsc-worker и boot-протокол с reconcile-циклом — обвязка поверх Runtime::process.
- M9: Presenter-мок; биометрия платформ — device-код.

- Composer часть 2 (C1: Tauri + vanilla TS UI, 6 экранов) — не в этом архиве: Tauri-сборке нужны системные webkit-библиотеки и Rust 1.77+.
- Proposal-формат: hardened TLV вместо CBOR из спеки §5.1 — сознательное отклонение (одно семейство парсеров с mu-wire); при требовании строго CBOR — замена изолирована в proposal.rs.
- endpoint.rs по спеке живёт в демоне; здесь как референс той же mu-policy-валидации с обеих сторон.

## Соответствие спекам
Каждая RISK-секция из SPEC_M4_M7a_M8_v1 имеет тест с тем же идентификатором в комментарии.
