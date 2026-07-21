> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Avalanche P-Chain API FAQ

> Source: [https://www.alchemy.com/docs/reference/avalanche-p-api-faq.md](https://www.alchemy.com/docs/reference/avalanche-p-api-faq.md)

# Avalanche P-Chain API FAQ

> Frequently asked questions about the Avalanche Platform Chain (P-Chain)

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is the Avalanche P-Chain?

The Avalanche Platform Chain (P-Chain) is the metadata chain on Avalanche that maintains the validator set, handles staking and delegation, and coordinates the creation of subnets and blockchains. It is one of the three primary chains (P-Chain, X-Chain, C-Chain) on the Avalanche network.

## What is the Avalanche P-Chain API?

The Avalanche P-Chain API is a JSON-RPC interface that lets you interact with the Avalanche P-Chain. It provides methods such as `platform.getCurrentValidators`, `platform.getHeight`, `platform.getSubnets`, and `platform.getMinStake` for querying validators, blocks, subnets, and staking-related data.

## How can I get started using the Avalanche P-Chain API?

Explained in [Avalanche P-Chain API Quickstart](/docs/reference/avalanche-p-api-quickstart).

## Is the Avalanche P-Chain EVM compatible?

No. The Avalanche P-Chain is not EVM compatible. It uses the `platform.*` JSON-RPC namespace. For EVM-compatible smart contracts and dApps, use the [Avalanche C-Chain](/docs/reference/avalanche-api-quickstart).

## What API does the Avalanche P-Chain use?

The Avalanche P-Chain uses the JSON-RPC 2.0 API standard. Parameters are passed by name (as a JSON object). The same base URLs as the Avalanche C-Chain are used, with `/ext/bc/P` appended after your API key in the path.

## What is an Avalanche API key?

When accessing Avalanche (including the Avalanche P-Chain) via a node provider like Alchemy, you use an API key to authenticate requests.

For the best development experience, we recommend that you [sign up for a free API key](https://dashboard.alchemy.com/signup)!

## Which libraries support the Avalanche P-Chain?

The P-Chain is not EVM-based, so Ethereum libraries like ethers.js or viem do not apply to `platform.*` methods. You can call the API with any HTTP client (e.g. `curl`, `fetch`, or a generic JSON-RPC client) using the Alchemy endpoint and JSON-RPC 2.0 request format.

## What programming languages work with the Avalanche P-Chain?

Any language that can send HTTP POST requests and handle JSON works with the Avalanche P-Chain API (e.g. JavaScript, TypeScript, Python, Go, Shell). There is no dedicated P-Chain SDK from Alchemy; use the documented JSON-RPC methods with your preferred client.

## What does the Avalanche P-Chain use for gas?

The Avalanche P-Chain uses AVAX (and nAVAX) for staking, delegation, and transaction fees. Fee configuration can be queried via `platform.getFeeConfig` and `platform.getFeeState`.

## Supported Avalanche P-Chain methods

You can find the list of supported Avalanche P-Chain methods on the [Avalanche P-Chain API Overview](/docs/avalanche-p/avalanche-p-api-overview) page.

## My question isn't here, where can I get help?

If you have any questions or feedback, please contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.