# ksg-pro-anchor — policy for paying the anchors of a keysingate Pro capsule

The owner's rules for the gas of anchors (Solana): how much one anchor may cost, how much one capsule
and all together may spend in a day, which capsules may be paid for. One wallet pays for all of them (owner's
decision 07.11: no sub-wallet per capsule, no refill); keep 0.00089 SOL in it besides what the anchors cost
(a wallet cannot be left under the rent minimum). See `Workflow_muGate_gaz_Pro` §4 and Spec S5.

| Form | File | What it is |
|---|---|---|
| C — description | `../ksg-pro-anchor.toml` | the policy as text, read by the daemon at start |
| A — limits only | *not made* | a Δ-Composer draft (`draft.bin`) |
| B — with the capsule list | *not made* | a Δ-Composer draft (`draft.bin`) |

**Forms A and B are not made, and why.** The Δ-Composer (`mu-compose`, `composer-tauri`) keeps a payment
policy: a limit in the minor units of a token, a whitelist of EVM addresses (`0x…`, a chain id), a resource
allowlist. An anchor's policy has other things in it — lamports, a fee cap per anchor, a budget per capsule, a
list of capsules by container multihash — and the composer has no place for them (`MU_CHAIN`
is 84532 or 8453). Drafts made there would either lose the rules or invent a mapping the owner did not choose.
Teaching the composer a second kind of policy is a piece of work of its own (recorded as gap Г-23); until
then the owner edits the TOML (form C).

## Use

```bash
# edit: the cluster, the limits, and the capsule ids (ksg-capsule --home DIR status → "id")
$EDITOR policy-templates/ksg-pro-anchor.toml
# the daemon refuses to start on a wrong value or an unknown key and prints which
mu-daemon init-ksg --template policy-templates/ksg-pro-anchor.toml …
```

A capsule entry that still holds `PLACEHOLDER…` is skipped with the warning `W-CAP-01`; with no capsule
named, every anchor is denied as `not_allowed` (`W-CAP-02`).
