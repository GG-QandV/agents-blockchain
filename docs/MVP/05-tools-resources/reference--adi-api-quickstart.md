> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# ADI API Quickstart

> Source: [https://www.alchemy.com/docs/reference/adi-api-quickstart.md](https://www.alchemy.com/docs/reference/adi-api-quickstart.md)

# ADI API Quickstart

> How to get started building on ADI using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

ADI Network is an EVM-compatible Layer 2 that leverages cryptographic Zero-Knowledge validity proofs (ZKPs) to ensure both security and efficiency in transaction processing. ADI is enabling seamless integration between traditional finance, crypto ecosystems, and regulated markets.

The ADI API allows interaction with the ADI network through a set of JSON-RPC methods. Its design is familiar to developers who have worked with Ethereum's JSON-RPC APIs, making it intuitive and straightforward to use.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create an ADI client connected to Alchemy and fetch the latest block number!

<CodeGroup>
  ```text npm
  npm install --save viem
  ```

  ```text yarn
  yarn add viem
  ```
</CodeGroup>

## Create a client connected to Alchemy

<CodeGroup>
```js
import { createPublicClient, http, defineChain } from "viem";

const adiTestnet = defineChain({
  id: 12227332,
  name: "ADI Testnet",
  nativeCurrency: { name: "ETH", symbol: "ETH", decimals: 18 },
  rpcUrls: {
    default: { http: ["https://adi-testnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"] },
  },
});

const client = createPublicClient({
  chain: adiTestnet,
  transport: http("https://adi-testnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
});
```
</CodeGroup>

Now that you've created a client connected to Alchemy, you can continue with some basics:

## Get the latest block number

<CodeGroup>
```js
const blockNumber = await client.getBlockNumber();
console.log("Current block number:", blockNumber);
```
</CodeGroup>

## Get an address balance

> 📄 **This content also appears in [Abstract API Quickstart](05-tools-resources/reference--abstract-api-quickstart.md)** — see there for full details.

## Read block data

<CodeGroup>
```js
const block = await client.getBlock({
  blockNumber: blockNumber, // from previous example
});
console.log(block);
```
</CodeGroup>

## Fetch a transaction by hash

<CodeGroup> 
```js 
const tx = await client.getTransaction({ hash: "0xYOUR_TX_HASH" });
console.log(tx);
```
</CodeGroup>

## Fetch a transaction receipt

<CodeGroup>
```js
const receipt = await client.getTransactionReceipt({
  hash: "0xYOUR_TX_HASH"
});
console.log(receipt);
```
</CodeGroup>

# ADI APIs

For the full list of ADI APIs, see the [ADI API Endpoints](/docs/chains#adi-apis).