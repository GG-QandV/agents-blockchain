> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Ethereum API Quickstart

> Source: [https://www.alchemy.com/docs/reference/ethereum-api-quickstart.md](https://www.alchemy.com/docs/reference/ethereum-api-quickstart.md)

# Ethereum API Quickstart

> How to get started building on Ethereum using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

The Ethereum API lets your application connect to an Ethereum node that is part of the Ethereum blockchain. You can interact with onchain data and send different types of transactions to the network using the endpoints provided by the API.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create an Ethereum client connected to Alchemy and fetch the latest block number!

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
import { createPublicClient, http } from "viem"; 
import { mainnet } from "viem/chains";

const client = createPublicClient({
  chain: mainnet,
  transport: http("https://eth-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Ethereum tutorials

Check out the following tutorials to learn how to build with Ethereum:

* [Build a Web3 Dashboard](/docs/web3-dashboard-prompt)
* [What is Proof of Stake?](/docs/what-is-proof-of-stake)
* [How do I distinguish between a contract address and wallet address?](/docs/reference/ethereum-api-faq#contract-vs-wallet-address)

For full documentation on Web3 libraries, check out the official documentation:

* [Viem Documentation](https://viem.sh) - Modern TypeScript interface for Ethereum
* [Ethers.js Documentation](https://docs.ethers.org) - Complete Ethereum wallet implementation