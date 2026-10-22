# μ Gate KSG — changelog

## 0.2.1-ksg — 2026-10-20 — journal format 2

### Changed (journal format 2)
- New `Kind::PendingAuth` record (tag 12): `intent_hash ‖ amount_total:u128 ‖ nonce[32] ‖ valid_after:u64 ‖ valid_before:u64`.
  Both time bounds are stored, so a retry signs the same digest and yields the same header.
  x402 writes it instead of `Pending`, so the single-use authorization nonce is on disk
  before the money moves. Old journals and `Pending` records (Sui/EVM) are read as before;
  the hash chain stays valid because the new tag has its own offsets.
- `Connector::pre_auth()` and `Connector::execute_auth(intent, signer, Option<AuthParams>)`.
  x402 returns `true` and refuses to sign without parameters written to the WAL; it no longer
  invents a nonce inside `execute`.
- The 402 offer travels **with the execution**, not in connector state: `ExecCtx { auth: Option<AuthParams>, offer: Option<X402Challenge> }`. `X402Connector::set_challenge()` is gone, so an offer of the previous intent cannot leak into the next payment.
  No offer → `Rejected(NoChallenge)` and nothing is signed. The window is exactly
  `maxTimeoutSeconds` (no 300 s default), never more than an hour; a WAL window
  longer than the offer is refused.
- One offer pays once: a second payment on the same 402 answer is
  `Rejected(OfferAlreadyUsed)`. The `X-PAYMENT` header stays one-shot, and a failed
  attempt clears it.
- x402 `status` reads `authorizationState(authorizer, nonce)` from **two** RPC nodes
  (`MU_X402_RPC_1`, `MU_X402_RPC_2`). Nodes disagreeing, one node missing or an
  unavailable node → `Unknown`: the reserve is kept, nothing is released on a guess.
  The previous time-window guess is gone.
- Reconcile on start: `PendingAuth` → connector status; used → `Settled` (nonce is the chain key),
  failed → `Failed`, still pending after `valid_before + 60 s` → `Failed` (reserve released).
  Two nodes disagreeing or unavailable → reserve kept, `Alert 0x0A02`.
- Policy endpoint for Composer: `<MU_HOME>/mu-policy.sock`, framing `len:u32 LE ‖ body`
  (the same as `composer-core::client`). A proposal is decoded, validated against Ω,
  checked against the current Δ hash (TOCTOU), shown as a diff and applied **only** after
  the owner confirms on the daemon's stdin; a refusal leaves Δ untouched and answers
  `rejected: human_denied`. An applied change is logged as `DeltaChanged`.
- The owner proof hash now binds the connector and the resource, so two intents that differ
  only there no longer share one proof.

### Tests
- The x402 self-check is pinned by a test with a challenge naming a *foreign* recipient
  (the previous test compared the intent with itself and could not fail).

### Policy: the journal is the source of truth
- `Kind::DeltaApplied` (tag 13) records each change with the **full canonical Δ** and an
  owner signature over `(version, prev_hash, next_hash)`. `policy.toml` is only a cache.
- Order of a change: owner signature → journal (fsync inside `append`) → atomic
  `policy.toml` replace (tmp, fsync, rename, fsync dir) → μ advanced to the new tail.
- Recovery reads the last `DeltaApplied`: the file matching it is used as is; a file that
  lags but matches the base or an earlier journal version is rewritten from the journal;
  a file that is newer, matches no journal version, or carries a broken signature makes the
  daemon refuse to start. A planted `policy.toml` with looser limits never takes effect.
- `delta_hash` now covers `resource_allowlist` (it was silently ignored), so two Δ that
  differ only in resource rules no longer share a hash.
- The owner sees a full diff before confirming: both limits, every added and removed
  whitelist address with its label, and every added, removed or repriced resource rule.
- μ is advanced to the log tail after every append (`Mu::advanced`). Before this, any
  payment left μ.log_head behind the chain, and the next start failed with `log_head
  mismatch` — the daemon could not be restarted after its first payment.

### Real settlement lookup (batch B)
- x402 `status` no longer asks `authorizationState`. It searches `eth_getLogs` for
  `AuthorizationUsed(address,bytes32)` (topic0 `0x98de5035…b10a5`, topic1 the payer, topic2 the
  authorization nonce) on both RPC nodes, over a window of `MU_X402_LOG_WINDOW` blocks
  (default 5000), then reads the receipt.
- `Settled` only when both nodes return the same transaction, both receipts are `0x1`, and the
  transfer has at least `MU_X402_CONFIRMATIONS` confirmations (default 12) on the smaller of
  the two heads. Otherwise the record stays pending and reconcile writes `AlertAbout`.
- `block` in `Settled` is now the real block of `AuthorizationUsed`; `effective_gas` stays 0 by
  design, because the facilitator pays the gas.
- The topic constant is pinned by a test that recomputes its keccak, and the live Base Sepolia
  tx is the fixture.

### Wire v3/v4: the header reaches the agent (batch A)
- Payload v3 = `v=3 ‖ recipient ‖ amount ‖ chain_id ‖ agent ‖ nonce ‖ ts ‖ resource ‖
  offer_len:u16 BE ‖ offer JSON`. The 402 offer is inside the signed bytes, at most 2048
  bytes, and parsed strictly: unknown keys are refused.
- Frame v4 = `v=4 ‖ agent ‖ nonce ‖ ts ‖ ref_nonce:u64`, signed by the agent. It asks for the
  header of an earlier request; the request nonce is not consumed, so repeating it is safe.
- New `Kind::AuthHeader` (tag 16): agent, request nonce, intent hash, authorization nonce,
  `valid_before`, header. Written with fsync right after signing; the agent gets the header
  only after this record exists.
- `X402Connector::execute_auth` returns `Executed { tx, header }`. Connector state for the
  header (`last_auth`, `take_header`) is gone — the same reason `set_challenge` went.
- Wire answers: `0x21 ‖ len:u16 LE ‖ header` (Authorized), `0x22` (expired), `0x23` (no such
  record or another agent's). `IntentStatus::Authorized` carries the header, the nonce and
  `valid_before` out of the pipeline.

### Safe defaults (batch C)
- The daemon refuses to start without `MU_HOME` and `MU_CHAIN_ID`; there are no silent
  fallbacks to `/tmp/mu` or chain 8453.
- With `MU_CONNECTOR=x402`, `MU_X402_NETWORK` is required (`base` | `base-sepolia`) and its
  chain must equal `MU_CHAIN_ID`, so a mainnet network can never run with testnet intents.
- Key files are already refused above mode 600 by `SoftVault::from_files`; the daemon-level
  test now pins that refusal.

### Alerts
- `Kind::AlertAbout` (tag 14) ties an alert to an intent. `reconcile` writes it once per
  `(code, intent)` transition; a restart with the same unresolved reserve reports on stderr
  and does not grow the journal.

### Legacy records
- A `Pending` record with `chain_nonce = 0` has no on-chain key and cannot be verified. After
  `3600 s + 60 s` from its timestamp it is closed `Failed` with `Alert 0x0A01`. This releases
  the reserve without checking the chain: the daemon never returned the payment header for
  those records, so no money moved on them.


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
