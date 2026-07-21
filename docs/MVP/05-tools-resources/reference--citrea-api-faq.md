> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Citrea API FAQ

> Source: [https://www.alchemy.com/docs/reference/citrea-api-faq.md](https://www.alchemy.com/docs/reference/citrea-api-faq.md)

# Citrea API FAQ

> Frequently asked questions about the Citrea API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Citrea?
Citrea is a Bitcoin L2 zk-rollup with a Type-2 zkEVM that keeps data availability and settlement on Bitcoin—via its BitVM-based “Clementine” two-way peg—bringing full EVM compatibility to BTC.

## How do I get started with Citrea?
Check out our [Citrea API Quickstart guide](/docs/reference/citrea-api-quickstart) to get started building on Citrea.

## What is the Citrea API?
The Citrea API lets you interface with the Citrea testnet (mainnet coming soon). With this API, you can execute transactions, query onchain data, and interact with the Citrea network using the JSON-RPC standard.

## Is Citrea EVM compatible?
Yes, Citrea is EVM compatible.

## What API does Citrea use?
Citrea uses the JSON-RPC API standard. This API is essential for any blockchain interaction on the Citrea network, allowing you to read block/transaction data, query chain information, execute smart contracts, and store data onchain.

## What methods are supported on Citrea?
Citrea supports standard Ethereum JSON-RPC methods. Some chain-specific methods may vary. Please check the [Citrea API Endpoints](/docs/chains#citrea-apis) for a complete list.

## What is a Citrea API key?
When accessing the Citrea network via a node provider like Alchemy, you use an API key to send transactions and retrieve data from the network. For the best development experience, we recommend that you [sign up for a free API key](https://dashboard.alchemy.com/signup).

## Which libraries support Citrea?
Common Ethereum libraries like [ethers.js](https://docs.ethers.org/v5/) should be compatible with Citrea, given its EVM nature.

> 📄 **This content also appears in [ADI API FAQ](05-tools-resources/reference--adi-api-faq.md)** — see there for full details.