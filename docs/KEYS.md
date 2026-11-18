# μ Gate — keys: create, use, store

μ Gate keeps the key an agent pays from **out of the agent's reach**. This guide covers the keys
themselves. It is written for the file-backed vault the daemon has today (keys in files, held in
process memory); hardware backends (Secure Enclave, StrongBox, TPM) are not implemented yet.

## What is what

| File (in `$MU_HOME`) | Secret? | What it does | If you lose it | If someone else gets it |
| --- | --- | --- | --- | --- |
| `wallet.key` | **yes** | The wallet the money is spent from (secp256k1, EVM). | The funds in that wallet are gone. | They can spend all of it. **No rule applies to them** — the gate only guards the agent. |
| `owner.key` | **yes** | Signs the policy (Δ) and your confirmations. | You can't change the policy; start a new home. | They can sign a looser policy / approve payments as you. |
| `mu.key` | **yes** | Signs the μ object and every log entry. | The daemon refuses to start; start a new home. | They can forge log entries. |
| `agent.key` | yes, **the agent's** | The agent's identity for the socket (ed25519). | Make a new home or re-issue the agent. | They can send intents as the agent (still bounded by your policy). |
| `agent.pub` | no | The agent's public key; the daemon's allowlist. | Re-derive from `agent.key`. | — |
| `mu.bin` | no | The owner-signed μ object: your policy (limit, threshold, whitelist, ceiling). | Re-issue (new home). | Can't be forged: signed. |
| `log.mulog` | no | Append-only signed hash chain: every quote, decision and payment. | Audit trail lost; the daemon won't start without a log matching `mu.bin`. | Read-only meaning: it shows payees and amounts. |

The daemon never prints a private key; `mu-keygen show` prints public values only.

## Build

```bash
cd mu
cargo build --release -p mu-daemon -p mu-keygen --features mu-daemon/softvault
# binaries: target/release/mu-keygen, target/release/mu-daemon
```

Rust ≥ 1.75. Run `cargo test --workspace --features mu-vault/softvault` first if you want to see the
suite pass on your machine.

## Create

```bash
mu-keygen init ~/.mu-gate --limit 25 --threshold 25 --ceiling 1000 --chain 8453 \
  --allow 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed=my-api
```

| Option | Meaning |
| --- | --- |
| `--limit` | Daily limit, USDC (default 0) |
| `--threshold` | Owner confirmation above this, USDC (default 0). Keep it at or above `--limit` for now: the daemon's confirmation dialog is a stub that denies (see Limits) |
| `--ceiling` | Hard cap per payment, USDC (default 1000) |
| `--chain` | 8453 Base (default), 84532 Base Sepolia |
| `--allow ADDR[=label]` | A recipient on the whitelist; repeat for each |

- Keys come from the operating system's random generator and are written with mode **600** inside a
  directory of mode **700**, from the first byte.
- It **refuses to overwrite** anything: use an empty or new directory.
- A mistake in the policy (a bad `--allow` address) stops it before any key is written.
- With no `--allow`, nothing can be paid: the whitelist is empty. With no `--limit`, the limit is 0.
  That is the safe default; say what you want to allow.

Check and look at it later, any time:

```bash
mu-keygen check ~/.mu-gate    # permissions, the μ object verifies, the log chain is intact
mu-keygen show  ~/.mu-gate    # wallet address, public keys, policy (no secrets)
```

Always test on **Base Sepolia** (`--chain 84532`, `MU_X402_NETWORK=base-sepolia`) before Base.

## Use

1. **Back up first** (below). Then fund the wallet address that `init` printed with a **small** amount.
2. Run the daemon:
   ```bash
   MU_HOME=~/.mu-gate mu-daemon                     # stub connector: no real payments
   MU_HOME=~/.mu-gate MU_CONNECTOR=x402 mu-daemon   # sign real x402 authorizations (see limits)
   ```
   `MU_SOCK` sets the socket (default `/tmp/mu-daemon.sock`, mode 600). If `MU_WALLET_ADDR` is set it
   must equal the wallet of `wallet.key`, or the daemon stops.
3. Give the **agent** only `agent.key`, and run the agent as a **different OS user**, or in a
   container, that cannot read `$MU_HOME`. If the agent can read `wallet.key` the gate is decoration.
   The sample client signs a frame with it:
   ```bash
   python3 clients/mu-client.py /tmp/mu-daemon.sock <recipient> <amount_minor> agent-1 --key agent.key
   ```
   (`amount_minor` is in USDC's six decimals: 1000000 = 1 USDC.)
4. What the gate does with each intent: quote → ceiling → whitelist, daily window, resource price cap →
   owner confirmation above the threshold → write-ahead log → sign. Every step is in `log.mulog`,
   including what was quoted and why a denial happened.

### Change the policy

Today the policy lives in the owner-signed `mu.bin`. To change it, make a **new** home with
`mu-keygen init` (and move funds with your own wallet tooling); updating a live home is not wired
into the daemon yet.

## Store

- **The three secrets (`wallet.key`, `owner.key`, `mu.key`)**: keep one **offline** copy, encrypted.
  For example, with `age`:
  ```bash
  tar -C ~/.mu-gate -cf - wallet.key owner.key mu.key | age -p > mu-gate-keys-$(date +%F).age
  ```
  Store it on a USB stick or in a password manager's secure notes, **not** in the same place as the
  machine, and test restoring it once (`age -d`, then `mu-keygen check`).
- **Never** commit keys, paste them in chats, put them in env vars, shell history or CI logs, or
  sync `$MU_HOME` to a cloud drive unencrypted. A private key is only ever a file; the daemon and
  `mu-keygen` refuse key files readable by anyone else.
- **Disk encryption** on the machine that runs the daemon (LUKS / FileVault). The key is in the
  daemon's memory while it runs; anyone with root on that host can reach it. The gate protects you
  from the **agent**, not from a compromised host.
- **Keep the wallet small.** `wallet.key` is a hot key: top it up for what the daily limit needs,
  not your savings.
- Back up `log.mulog` too if you want the audit trail; it is not secret but is yours.
- Lost `wallet.key` = lost funds. There is no recovery. Rotate by creating a new home, moving the
  funds, and retiring the old files (`shred -u` where your filesystem allows; on SSDs, rely on disk
  encryption).

## Limits of this version (read before using real money)

- The vault is a **software vault**: keys sit in process memory; not hardware-backed.
- The owner-confirmation dialog in `mu-daemon` is a stub that **denies**: payments above the
  threshold are refused, never approved. Keep `--threshold` at or above `--limit`.
- `MU_CONNECTOR=x402` produces a signed authorization but the daemon does not return it to the agent
  yet; the end-to-end paying flow today is `ksg-smoke` (`MU_PAYER_KEY_FILE`, Base Sepolia), see
  `mu/CHANGELOG-ksg.md`. Intents are routed as `BankStub` in the daemon until that lands.
- One agent (`agent-1`) per home.
- `mu-fixtures` and `ksg-smoke`'s fallback key use **public test seeds**. Never send funds to them.
