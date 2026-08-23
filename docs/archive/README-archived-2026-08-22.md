# μ Gate — Don't give your AI agent your wallet. Put a rule between them.

[![Version](https://img.shields.io/badge/version-0.2.1-blue)](https://github.com/GG-QandV/agents-blockchain)
[![License](https://img.shields.io/badge/license-BSL%201.1-orange)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://rust-lang.org)

**μ Gate** is a local daemon that controls AI-agent payments. The agent never sees the wallet key. The owner sets rules (daily budget, whitelist, per-resource price caps). Every decision and payment is logged in an append-only hash chain.

**Zero gas fees** for the owner — payments use [x402](https://github.com/coinbase/x402) (EIP-3009 `TransferWithAuthorization`) over USDC on Base.

---

## Quickstart (30 min)

### Prerequisites

- **Rust** ≥ 1.75 (`rustup install 1.75`)
- **Linux** or **macOS** (Unix socket transport; Windows via WSL2)
- **Python 3** for the CP1 test harness (optional)

### 1. Build & test

```bash
cd mu
cargo test --workspace --features mu-vault/softvault
cargo build -p mu-daemon --features softvault
```

### 2. Generate fixtures

```bash
export MU_HOME=/tmp/mu-gate
rm -rf $MU_HOME
cargo run -p mu-fixtures
```

### 3. Run the daemon (stub mode — no real payments)

```bash
./target/debug/mu-daemon
```

In another terminal:

```bash
python3 mu/clients/mu-client.py /tmp/mu-daemon.sock \
  0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 1000000 cp1-agent
```

### 4. Run CP1 (real payment proof — needs funded wallet)

```bash
# Fund wallet with $20 USDC on Base mainnet
MU_CONNECTOR=x402 MU_WALLET_ADDR=0x<your_address> ./target/debug/mu-daemon

# In another terminal:
python3 mu/clients/cp1-harness.py
```

See `cp1-harness.py` for the 4 test cases (allowed call, wrong address denial, threshold test, log review).

---

## Architecture

![Agent-Connector architecture](diagram-1.svg)

*Architecture diagram: [`diagram-1.svg`](diagram-1.svg) (source: [`docs/diagram-1.drawio`](docs/diagram-1.drawio))*

### Modules

| Crate                   | Role                                                      |
| ----------------------- | --------------------------------------------------------- |
| `mu-gate`               | M8 — unix socket, ed25519 auth, rate-limit, allowlist     |
| `mu-runtime`            | M6 — pipeline: Ω → Δ → [human] → WAL → execute            |
| `mu-connect` / `x402`   | M7x — x402 client (EIP-712, EIP-3009, self-check)         |
| `mu-connect` / `crypto` | M7a — EVM USDC connector (Base, legacy)                   |
| `mu-policy`             | M2/M3 — Ω (ceiling) + Δ (whitelist, budget, resources)    |
| `mu-log`                | M5 — append-only hash chain WAL, reconcile                |
| `mu-vault`              | M4 — k256/P-256 keys, soft vault (dev) / enclave (TBD)    |
| `mu-human`              | M9 — owner biometric confirmation dialog                  |
| `mu-core`               | M1 — μ-object format, CBOR-like serialization             |
| `mu-daemon`             | Binary — boot protocol, socket server, Runtime            |
| `mu-license`            | Ed25519-signed license check                              |
| `composer-core`         | C2 — Delta proposal encode/decode (offline policy change) |
| `composer-cli`          | C2 — CLI tool for whitelist/daily-limit management        |

---

## Configuration

### `policy.toml`

```toml
daily_limit_minor = 5_000_000_000        # 5000 USDC/day
confirm_threshold_minor = 100_000_000    # >100 USDC → owner confirms

[[whitelist]]
address = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
label = "CoinGecko x402"

[[resource_allowlist]]
host = "api.coingecko.com"
path_prefix = "/api/v3/simple/price"
max_price_per_call_minor = 1_000_000     # $1/call
```

Templates in `mu/policy-templates/`:

- `approved-recipients-only.toml`
- `daily-budget.toml`
- `approval-above-threshold.toml`
- `cp1-coingecko.toml`

### Environment

| Env var          | Default   | Description                                |
| ---------------- | --------- | ------------------------------------------ |
| `MU_CONNECTOR`   | `stub`    | `x402` for real payments                   |
| `MU_WALLET_ADDR` | —         | Wallet address (hex, with or without `0x`) |
| `MU_HOME`        | `/tmp/mu` | Fixtures directory                         |
| `MU_POLICY_SOCK` | —         | Unix socket path for Composer              |

---

## Clients

- **Python** — `mu/clients/mu-client.py`
- **TypeScript** — `mu/clients/mu-client.ts`
- **CP1 harness** — `mu/clients/cp1-harness.py`

---

## License

**BSL 1.1** — Business Source License.

- **Personal use**: Free (full functionality)
- **Internal business use**: Free (see Additional Use Grant)
- **Competing product or SaaS**: Requires a commercial license
- **After 4 years (or Change Date 2030-07-15)**: Converts to Apache 2.0

See [LICENSE](./LICENSE) for full terms.
