> ⚠️ **This page is a template variant.** The consolidated content is in [api-faq](../00-consolidated/api-faq.md).
> Below is the original chain-specific version.

# Aptos API FAQ

> Source: [https://www.alchemy.com/docs/reference/aptos-api-faq.md](https://www.alchemy.com/docs/reference/aptos-api-faq.md)

# Aptos API FAQ

> Frequently asked questions about the Aptos API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

## What is Aptos?

Aptos is a Layer 1 blockchain built for scalability, safety, and upgradeability. It uses the Move programming language and the AptosBFT consensus protocol to enable high-throughput and low-latency transactions while maintaining strong security guarantees.

## What is the Aptos API?

The Aptos API allows you to interact with the Aptos blockchain via a set of RESTful endpoints. It enables actions such as querying accounts and transactions, submitting new transactions, interacting with smart contracts, and accessing onchain resources.

## How can I get started using the Aptos API?

Take a look at our [Aptos API Quickstart](/docs/reference/aptos-api-quickstart) guide.

## Does Aptos support smart contracts?

Yes. Aptos supports smart contracts written in Move, a secure and expressive programming language developed specifically for the blockchain. You can deploy, call, and interact with smart contracts via the API.

## What API standard does Aptos use?

The Aptos API follows REST principles and communicates via standard HTTP requests and JSON responses. Endpoints are organized by functionality — accounts, blocks, transactions, events, etc.

## What is an Aptos API key?

When using a blockchain infrastructure provider like Alchemy, an API key allows you to send authenticated requests to the Aptos network. This lets you read data from the chain and submit transactions securely.

## Which libraries can I use with the Aptos API?

You can use any HTTP client that supports standard REST calls. Popular choices include:

* JavaScript: `axios`, `fetch`
* Python: `requests`
* Curl (for CLI or scripting)
* Community SDKs like `aptos-sdk`, `aptos-client` for Python and TypeScript

## What programming languages are compatible with the API?

The API works with any language that supports HTTP — such as JavaScript/TypeScript, Python, Go, Rust, and Java. You can integrate it into backend services, CLI tools, and dApps.

## What is used for fees in Aptos?

Aptos uses its native token, APT, to pay transaction fees. The API provides endpoints to estimate gas usage and fee costs before submitting a transaction.

## My question isn't listed here — where can I get help?

If you have any questions or feedback, contact us at support@alchemy.com or open a ticket in the Alchemy Dashboard.