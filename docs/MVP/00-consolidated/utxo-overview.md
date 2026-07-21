# Utxo Overview

**3 chains** · template-based (1 doc replaces 3 per-chain files)

## Common Template

The following content is **identical** across all chains:

# UTXO Overview

# UTXO Overview

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## UTXO-based model

## What is a UTXO?

A UTXO is the output of a previous transaction that has not yet been spent. Every time a transaction is confirmed:

## Properties of UTXOs

## Account model vs UTXO model

## Querying UTXO data with Alchemy

## Per-Chain Parameters

| Chain | Title | Rpc Url |
|-------|---|---|
| Bitcoin/Utxo | UTXO Overview | https://www.alchemy.com/docs/bitcoin/utxo.md](https:... |
| Bitcoincash/Utxo | UTXO Overview | https://www.alchemy.com/docs/bitcoincash/utxo.md](ht... |
| Litecoin/Utxo | UTXO Overview | https://www.alchemy.com/docs/litecoin/utxo.md](https... |

## Notes

- Template applies to all 3 chains with per-chain variable substitution only
- Per-chain values stored in `utxo-overview_data.json`
- Chains with notable unique content (beyond name substitution):
  - Bitcoin/Utxo: ~30 unique lines
  - Bitcoincash/Utxo: ~25 unique lines
  - Litecoin/Utxo: ~25 unique lines

---
Source: 00-consolidated/utxo-overview.md
