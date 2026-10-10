# ksg-bare-repeat — policy for paying a repeated keysingate `bare`

Three forms of one policy (owner's choice 2026-10-10: A, B and C).

| Form | File | Opens in | What it holds |
|---|---|---|---|
| A — limits only | `A-limits/draft.bin` | Δ-Composer (`mu-compose`, composer-tauri) | daily limit 5 USDC, every payment confirmed by the owner, empty whitelist (warning W-WL-01) |
| B — with the issuer | `B-whitelist/draft.bin` | Δ-Composer | A + the issuer's account in the whitelist (placeholder `0x…dEaD` — replace) |
| C — description | `../ksg-bare-repeat.toml` | a text editor | the same policy plus the resource allowlist (host / path / price cap) |

Open a draft (Base Sepolia, test network):

```bash
MU_CHAIN=84532 MU_HOME=<path>/ksg-bare-repeat/B-whitelist mu-compose show
```

Replace the placeholder with the issuer's real account:

```bash
export MU_CHAIN=84532 MU_HOME=<path>/ksg-bare-repeat/B-whitelist
mu-compose wl-rm 0x000000000000000000000000000000000000dEaD
mu-compose wl-add <issuer EIP-55 address> "KSG issuer"
```

`MU_CHAIN` — 84532 (Base Sepolia) or 8453 (Base, real money; default). The whitelist entry
is the x402 `payTo` (the issuer), not the USDC contract: the token is fixed per network in
the x402 connector and is never taken from a policy or a 402 answer.
