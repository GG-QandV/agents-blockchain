# μ Gate — edition keysingate (в.keysingate)

**Status:** skeleton, no code yet. Created 2026-09-23 as a separate edition so it
is not confused with the base μ Gate in `mu/`.
**Owner's task:** adapt μ Gate so it can pay Solana gas for keysingate anchoring
(keysingate `internal/TODO_na_potom.md`, T-1).

## What differs from the base μ Gate

| | Base μ Gate (`mu/`) | Edition keysingate |
|---|---|---|
| What the agent asks for | a **payment**: `Intent { recipient, amount, chain_id }` (`mu-connect/src/lib.rs`) | an **anchor**: a Solana transaction carrying one SPL Memo line `ksg:cp:v1 <log_id> <tree_size> <root>` (keysingate `ksg-anchor/src/solana.rs`) |
| Who is paid | a resource (x402, USDC on Base) or a Sui transfer | the Solana network fee (5,000 lamports per signature plus optional priority fee) |
| Connectors | `crypto` (EVM/Base), `sui`, `stub` | a Solana connector — absent in the base |
| Rules | daily budget, whitelist, per-resource price caps | anchoring budget per period, cap on fee per anchor, allowed memo prefix `ksg:cp:v1` only |

The base `Connector` trait moves money to a recipient. An anchor moves no money
to anyone; it only pays the fee. That is why this is an edition and not one more
connector: the intent type is different.

## What is reused unchanged

Everything that makes μ Gate what it is: the agent never sees the key
(`mu-vault`), rules are the owner's (`mu-policy`), every decision is logged in
the append-only hash chain (`mu-log`), and the connector invariants —
Unknown ≠ Failed (RISK-M7-1), settled only on agreement of two RPC nodes
(RISK-M7-3), the payload checked against the intent before signing (RISK-M7-5).
The edition links to these crates; it does not copy them.

## Plan (when T-1 is taken up)

1. `AnchorIntent { memo, fee_cap_lamports, cluster }`, validated against the
   allowed prefix before it reaches the signer — the analogue of RISK-M7-5.
2. Solana connector behind the same invariants as `sui.rs`: build the memo
   transaction, dry-run on two RPC nodes, check the memo and the fee against the
   intent, sign, submit, report the finalized slot.
3. Adapter on the keysingate side: `ksg-service::anchoring::ChainSender`
   implemented by calling this daemon, so the holder's wallet key stays in μ Gate.
4. Check whether x402 on Solana (USDC SPL) can take the fee off the holder the
   way EIP-3009 does on Base. Not verified.
5. Decide which record is the evidence of payment for an anchor: μ Gate's hash
   chain or the keysingate journal — they must not both claim it.
6. Licence check: this repository is BSL 1.1, keysingate `reference-impl` is
   Apache 2.0; the adapter in step 3 must not pull BSL code into Apache code.

## Not in this edition

Payments for resources (x402 on Base), Sui transfers, Composer. They stay in the
base μ Gate.
