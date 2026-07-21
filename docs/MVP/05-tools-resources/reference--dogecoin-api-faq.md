> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Dogecoin API FAQ

> Source: [https://www.alchemy.com/docs/reference/dogecoin-api-faq.md](https://www.alchemy.com/docs/reference/dogecoin-api-faq.md)

# Dogecoin API FAQ

> Frequently asked questions about the Dogecoin API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Dogecoin?

Dogecoin (DOGE) is a peer-to-peer cryptocurrency launched in 2013. Originally derived from Litecoin (and indirectly Bitcoin), it uses a Scrypt-based proof-of-work consensus and is known for its active community and low transaction fees.

## What is the Dogecoin API?

The Dogecoin API allows you to interact with the Dogecoin network using JSON-RPC methods. Through the API, you can retrieve data about blocks, transactions, and the mempool, as well as broadcast new transactions to the network.

## How can I get started using the Dogecoin API?

Check out our [Dogecoin API Quickstart](/docs/reference/dogecoin-api-quickstart) guide for setup instructions, sample code, and your first API call.

## Does Dogecoin support smart contracts?

Dogecoin supports basic Bitcoin-style scripting but does not have a full smart contract virtual machine like Ethereum.

## What API standard does Dogecoin use?

Dogecoin Core implements a standard JSON-RPC interface for querying blockchain data and submitting transactions, derived from Bitcoin Core.

## What is a Dogecoin API key?

When accessing the Dogecoin network via a node provider like Alchemy, you use an API key to send transactions and retrieve data from the network.

## Which libraries can I use with the Dogecoin API?

You can use any HTTP client that supports JSON payloads, e.g. `axios`, `fetch`, `requests`, or `curl`. Bitcoin-Core-compatible libraries like `bitcoin-core` (Node.js) and `python-bitcoinrpc` also work with Dogecoin's RPC interface.

## What programming languages are compatible with the API?

> 📄 **This content also appears in [Bitcoin API FAQ](05-tools-resources/reference--bitcoin-api-faq.md)** — see there for full details.

## What is used for fees in Dogecoin?

Dogecoin transaction fees are paid in DOGE. They are calculated per byte of transaction data and are used to incentivize miners to include transactions in blocks.

## What methods does Alchemy support for the Dogecoin API?

You can find a full list of supported JSON-RPC methods on the [Dogecoin API Overview](/docs/dogecoin/dogecoin-api-overview) page.

## My question isn't listed here, where can I get help?

If you have any questions or feedback, please contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.