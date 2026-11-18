# μ Gate KSG — changelog

## 0.2.1-ksg — 2026-11-18 (audit log: the log alone shows the gate held)

Additive: log tags 1–11 are unchanged; logs from 0.2.0-ksg open and verify, but their verdicts cannot be re-derived.

### Added
- `Kind::Quoted` (tag 12): recipient, chain, connector, amount, gas estimate, x402 resource and the hash of the Ω/Δ in force, written before any check. No record, no payment.
- `Kind::DenyReason` (tag 13): rule (`mu_log::deny::*`), total checked, cap met — after every denial, including owner refusal/timeout.
- `policy_hash(Ω, Δ)` (order-insensitive over connectors).
- Tests: `mu-log/tests/quoted_log.rs`, `mu-runtime/tests/audit_trail.rs` (denial reasons, paid chain, re-derivation from the log alone).

### Changed
- `intent_hash` now also covers connector and resource (hashes differ from older logs).
- An intent whose x402 resource exceeds 65 535 bytes is refused (`Failed`) before anything is logged.
- Outcome of a payment already sent (`Settled`/`Failed`): written with `Log::append_outcome` — retried, then queued inside the log, never dropped. While the queue is non-empty `Log::append` returns `Behind`, so no new intent starts; the queue is written first, in order, once the log is writable. The open `Pending` keeps holding the reserve meanwhile; the daemon prints an ALERT. A failed write is cut back to the last complete record (no half records mid-file). The queue is in memory: a crash before it is written leaves a `Pending` without outcome, as before.
- Denial and owner-decision records are no longer fire-and-forget: if one cannot be written, the intent returns `Failed` (and an approval that cannot be logged does not pay), instead of reporting a denial with a hole in the log.

## 0.2.0-ksg — 2026-10-10 (initial production baseline for keysingate)

Branch `keysingate`, tag `v0.2.0-ksg`. Base: μ Gate 0.2.1 (`baseline/evm-x402`) merged with the
English docs and the KSG edition notes of `master`.

### What it does with keysingate
An agent pays a keysingate issuer for a repeated `bare` (x402, USDC): the issuer's 402 answer →
Ω → Δ (whitelist, daily window, resource allowlist) → owner's confirmation → WAL → x402
connector → `X-PAYMENT`, which keysingate's seller side (`ksg-pay-v2`) accepts.

### Changed
- **x402 connector** (`mu-connect/src/x402.rs`), found wrong against an independent EIP-712
  implementation (eth-account 0.13.4) and fixed:
  - the EIP712Domain and TransferWithAuthorization type-hash constants were wrong — every
    signature of 0.2.1 was invalid; type hashes now come from the type strings and are pinned;
  - header is x402 version 1: `x402Version` 1, network from configuration, signature `0x` r‖s‖v
    with v 27/28, amounts and window as decimal strings;
  - payer address derived from the signing key; a key that is not the configured wallet signs
    nothing (`WalletMismatch`);
  - the 402 challenge must name exactly the intent's recipient, amount, network, the network's
    USDC contract and its EIP-712 domain, and a validity ≤ 1 h — else nothing is signed
    (`ChallengeMismatch`);
  - nonce from the OS RNG (was the clock);
  - networks: Base (8453) and Base Sepolia (84532, test network).
- **runtime**: x402 resource allowlist matched nothing — `host/path` lost its leading `/` and full
  URLs were not parsed (`split_resource`). The `cp1-coingecko` template now matches.
- **daemon**: `MU_X402_NETWORK=base|base-sepolia`.
- **Composer** (CLI and Tauri): chain from `MU_CHAIN` (8453 default, 84532).

### Added
- `mu-runtime/tests/ksg_x402_e2e.rs` — 4 E2E: paid with consent; owner says no; issuer not in the
  policy; daily window and price cap.
- `ksg-smoke` — pays a 402 answer from stdin through the whole pipeline (test wallet, Base Sepolia).
- Cross-repository E2E in keysingate: `scripts/e2e_ksg_x402.sh` (`MU_DIR=…/mu`).
- Policy templates `ksg-bare-repeat`: A (limits, Composer draft), B (issuer whitelist, Composer
  draft), C (TOML).

### Not in this release
- Paying Solana anchoring fees through μ Gate (KSG edition plan, `ksg/README.md`; keysingate
  T-1, postponed by the owner 23.09) — keysingate pays them itself.
- Settlement status from the chain (`authorizationState` via RPC): status follows the
  authorization's time window.
- Hardware vault backends — as in 0.2.1, SoftVault only; the Tauri Composer is not built in CI
  (needs webkit).

## 0.2.0-ksg — 11.10

- `ksg-smoke`: the payer's key from the file `MU_PAYER_KEY_FILE` (64 hex, mode 600, never printed; a file readable by others is refused) for one live payment on Base Sepolia; without it, the fixed test key as before.
