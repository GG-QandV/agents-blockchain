> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Litecoin API FAQ

> Source: [https://www.alchemy.com/docs/reference/litecoin-api-faq.md](https://www.alchemy.com/docs/reference/litecoin-api-faq.md)

# Litecoin API FAQ

> Frequently asked questions about the Litecoin API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Litecoin?

Litecoin (LTC) is a peer-to-peer cryptocurrency launched in 2011 by Charlie Lee. It is one of the oldest altcoins, derived from the Bitcoin codebase but with a faster block time (around 2.5 minutes) and the Scrypt proof-of-work hashing algorithm.

## What is the Litecoin API?

The Litecoin API allows you to interact with the Litecoin network using JSON-RPC methods. Through the API, you can retrieve data about blocks, transactions, and the mempool, as well as broadcast new transactions to the network.

## How can I get started using the Litecoin API?

Check out our [Litecoin API Quickstart](/docs/reference/litecoin-api-quickstart) guide for setup instructions, sample code, and your first API call.

## Does Litecoin support smart contracts?

Litecoin supports basic scripting, including SegWit and MWEB (MimbleWimble Extension Blocks) for privacy-preserving transactions, but does not have a full smart contract virtual machine like Ethereum.

## What API standard does Litecoin use?

Litecoin Core implements a standard JSON-RPC interface for querying blockchain data and submitting transactions, derived from Bitcoin Core.

## What is a Litecoin API key?

When accessing the Litecoin network via a node provider like Alchemy, you use an API key to send transactions and retrieve data from the network.

## Which libraries can I use with the Litecoin API?

You can use any HTTP client that supports JSON payloads, e.g. `axios`, `fetch`, `requests`, or `curl`. Bitcoin-Core-compatible libraries like `bitcoin-core` (Node.js) and `python-bitcoinrpc` also work with Litecoin's RPC interface.

## What programming languages are compatible with the API?

> 📄 **This content also appears in [Bitcoin API FAQ](05-tools-resources/reference--bitcoin-api-faq.md)** — see there for full details.

## What is used for fees in Litecoin?

Litecoin transaction fees are paid in LTC. They are calculated per virtual byte (vByte) of transaction data and are used to incentivize miners to include transactions in blocks.

## What methods does Alchemy support for the Litecoin API?

You can find a full list of supported JSON-RPC methods on the [Litecoin API Overview](/docs/litecoin/litecoin-api-overview) page.

## My question isn't listed here, where can I get help?

If you have any questions or feedback, please contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.