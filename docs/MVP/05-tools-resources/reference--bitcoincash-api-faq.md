> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Bitcoin Cash API FAQ

> Source: [https://www.alchemy.com/docs/reference/bitcoincash-api-faq.md](https://www.alchemy.com/docs/reference/bitcoincash-api-faq.md)

# Bitcoin Cash API FAQ

> Frequently asked questions about the Bitcoin Cash API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Bitcoin Cash?

Bitcoin Cash (BCH) is a peer-to-peer electronic cash system that originated as a hard fork of Bitcoin in 2017. It uses a larger block size to support higher on-chain transaction throughput, and is secured by proof-of-work mining.

## What is the Bitcoin Cash API?

The Bitcoin Cash API allows you to interact with the Bitcoin Cash network using JSON-RPC methods. Through the API, you can retrieve data about blocks, transactions, and the mempool, as well as broadcast new transactions to the network.

## How can I get started using the Bitcoin Cash API?

Check out our [Bitcoin Cash API Quickstart](/docs/reference/bitcoincash-api-quickstart) guide for setup instructions, sample code, and your first API call.

## Does Bitcoin Cash support smart contracts?

Bitcoin Cash supports a richer scripting language than Bitcoin via the CashScript ecosystem and CashTokens, but it does not have a full smart contract virtual machine like Ethereum.

## What API standard does Bitcoin Cash use?

Bitcoin Cash Node and BCHN implement a standard JSON-RPC interface for querying blockchain data and submitting transactions, derived from Bitcoin Core.

## What is a Bitcoin Cash API key?

When accessing the Bitcoin Cash network via a node provider like Alchemy, you use an API key to send transactions and retrieve data from the network.

## Which libraries can I use with the Bitcoin Cash API?

You can use any HTTP client that supports JSON payloads, e.g. `axios`, `fetch`, `requests`, or `curl`. There are also Bitcoin-Cash-specific libraries like `bchjs` (Node.js) and `bitcash` (Python) for deeper integration.

## What programming languages are compatible with the API?

> 📄 **This content also appears in [Bitcoin API FAQ](05-tools-resources/reference--bitcoin-api-faq.md)** — see there for full details.

## What is used for fees in Bitcoin Cash?

Bitcoin Cash transaction fees are paid in BCH and are calculated per byte of transaction data. Fees are used to incentivize miners to include transactions in blocks.

## What methods does Alchemy support for the Bitcoin Cash API?

You can find a full list of supported JSON-RPC methods on the [Bitcoin Cash API Overview](/docs/bitcoincash/bitcoincash-api-overview) page.

## My question isn't listed here, where can I get help?

If you have any questions or feedback, please contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.