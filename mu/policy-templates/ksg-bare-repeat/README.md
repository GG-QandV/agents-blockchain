# ksg-bare-repeat — Composer draft for paying a repeated keysingate `bare`

Opens in Δ-Composer: `MU_HOME=<this folder> mu-compose show` (or composer-tauri with the same `MU_HOME`).

| Field | Value | Why |
|---|---|---|
| daily_limit | 5 USDC | a repeated `bare` costs $0.1–1 |
| confirm_threshold | 0 | every payment confirmed by the owner while the issuer's account is not set |
| whitelist | empty | Composer takes 32-byte (Sui) addresses only (`mu-policy/src/sui_addr.rs:10`); USDC on Base and the issuer's account are 20-byte EVM addresses — rejected with `BadLength` |

Variant A of the owner's choice (2026-10-10): limits only. Variant B — teach Composer
EVM addresses for chain 8453 — would let the whitelist hold USDC Base and the issuer's account.
Regenerate: `MU_HOME=<this folder> mu-compose set-limit 5 && mu-compose set-threshold 0`.
