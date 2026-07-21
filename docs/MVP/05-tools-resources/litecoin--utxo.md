> ⚠️ **This page is a template variant.** The consolidated content is in [utxo-overview](../00-consolidated/utxo-overview.md).
> Below is the original chain-specific version.

# UTXO Overview

> Source: [https://www.alchemy.com/docs/litecoin/utxo.md](https://www.alchemy.com/docs/litecoin/utxo.md)

# UTXO Overview

> Understand how the Unspent Transaction Output (UTXO) model tracks balances on Litecoin.

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

Litecoin uses the **Unspent Transaction Output (UTXO) model** to track user balances, in contrast to the **account model** used by Ethereum and other EVM chains.

## UTXO-based model

> 📄 **This content also appears in [UTXO Overview](05-tools-resources/bitcoin--utxo.md)** — see there for full details.

When someone says they own 3 LTC, what they actually hold is one or more UTXOs that together sum to 3 LTC. To spend that balance, a wallet selects specific UTXOs as inputs to a new transaction.

## What is a UTXO?

A UTXO is the output of a previous transaction that has not yet been spent. Every time a transaction is confirmed:

> 📄 **This content also appears in [UTXO Overview](05-tools-resources/bitcoin--utxo.md)** — see there for full details.

> 📄 **This content also appears in [UTXO Overview](05-tools-resources/bitcoin--utxo.md)** — see there for full details.

## Properties of UTXOs

> 📄 **This content also appears in [UTXO Overview](05-tools-resources/bitcoin--utxo.md)** — see there for full details.

## Account model vs UTXO model

|                    | Account model (Ethereum, EVM chains)                        | UTXO model (Litecoin)                                                             |
| ------------------ | ----------------------------------------------------------- | --------------------------------------------------------------------------------- |
| User balance       | Single overall account state (e.g. address X holds 4.2 ETH) | Sum of specific UTXOs (e.g. address X owns 29 UTXOs totalling 2.65 LTC)           |
| Query model        | Direct: read account balance                                | Compute: aggregate over an address's UTXOs                                        |
| Privacy            | Repeated address use correlates activity                    | Rotating addresses per output is the recommended privacy model                    |
| Transaction model  | State transition: balance update                            | Input/output: consume UTXOs, create new ones                                      |

## Querying UTXO data with Alchemy

Alchemy's UTXO API gives you a clean REST interface to fetch Litecoin addresses, balances, balance history, UTXOs, and block/transaction data. See the UTXO API endpoints in the sidebar, or the [UTXO migration guide](https://www.alchemy.com/docs/bitcoin/utxo-migration-guide) if you're moving from QuickNode, BlockCypher, or Blockdaemon.