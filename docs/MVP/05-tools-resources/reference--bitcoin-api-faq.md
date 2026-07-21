> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Bitcoin API FAQ

> Source: [https://www.alchemy.com/docs/reference/bitcoin-api-faq.md](https://www.alchemy.com/docs/reference/bitcoin-api-faq.md)

# Bitcoin API FAQ

> Frequently asked questions about the Bitcoin API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Bitcoin?

Bitcoin is the first decentralized, peer-to-peer digital currency. It allows you to send and receive payments without the need for intermediaries like banks. Bitcoin uses a UTXO (Unspent Transaction Output) model and is secured by proof-of-work mining.

## What is the Bitcoin API?

The Bitcoin API allows you to interact with the Bitcoin network using JSON-RPC methods. Through the API, you can retrieve data about blocks, transactions, and the mempool, as well as broadcast new transactions to the network.

## How can I get started using the Bitcoin API?

Check out our [Bitcoin API Quickstart](/docs/reference/bitcoin-api-quickstart) guide for setup instructions, sample code, and your first API call.

## Does Bitcoin support smart contracts?

Not in the same way as EVM-based chains. Bitcoin supports basic scripting for transactions, but does not have a full smart contract virtual machine like Ethereum. More advanced contract-like functionality is being explored via protocols like Taproot and Ordinals.

## What API standard does Bitcoin use?

Bitcoin Core implements a standard JSON-RPC interface for querying blockchain data and submitting transactions.

## What is a Bitcoin API key?

When accessing the Bitcoin Chain network via a node provider like Alchemy, you use an API key to send transactions and retrieve data from the network.

## Which libraries can I use with the Bitcoin API?

You can use any HTTP client that supports JSON payloads — e.g., `axios`, `fetch`, `requests`, or `curl`. There are also Bitcoin-specific libraries like `bitcoin-core` (Node.js), `python-bitcoinrpc`, and `btcd` (Go) for deeper integration.

## What programming languages are compatible with the API?

The API works with any programming language that can send JSON over HTTP. Common choices include JavaScript/TypeScript, Python, Go, and Java.

## What is used for fees in Bitcoin?

Bitcoin transaction fees are paid in BTC. They are calculated per virtual byte (vByte) of transaction data and are used to incentivize miners to include transactions in blocks.

## What methods does Alchemy support for the Bitcoin API?

You can find a full list of supported JSON-RPC methods on the [Bitcoin API Endpoints](/docs/chains#bitcoin-apis) page.

## My question isn't listed here — where can I get help?

If you have any questions or feedback, please contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.