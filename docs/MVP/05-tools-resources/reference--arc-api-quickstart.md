> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Arc API Quickstart

> Source: [https://www.alchemy.com/docs/reference/arc-api-quickstart.md](https://www.alchemy.com/docs/reference/arc-api-quickstart.md)

# Arc API Quickstart

> How to get started building on Arc and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

*To use the Arc API you'll need to [create a free Alchemy account](https://dashboard.alchemy.com/signup) first!*

## Introduction

Arc is an open Layer 1 blockchain by Circle, purpose-built for stablecoin finance, featuring USDC as the native gas token, sub-second finality, and direct integration with Circle's platform for secure access to liquidity and developer tooling.

## What is the Arc API?

The Arc API allows interaction with the Arc network through a set of JSON-RPC methods. Its design is familiar to developers who have worked with Ethereum's JSON-RPC APIs, making it intuitive and straightforward to use.

## Getting started

### 1. Choose a package manager (npm or yarn)

Select a package manager to manage your project's dependencies. Choose between `npm` and `yarn` based on your preference or project requirements.

<CodeGroup>
  ```shell npm
  # Begin with npm by following the npm documentation
  # https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
  ```

  ```shell yarn
  # For yarn, refer to yarn's installation guide
  # https://classic.yarnpkg.com/lang/en/docs/install
  ```
</CodeGroup>

### 2. Set up your project

Open your terminal and execute the following commands to create and initialize your project:

<CodeGroup>
  ```shell npm
  mkdir arc-api-quickstart
  cd arc-api-quickstart
  npm init --yes
  ```

  ```shell yarn
  mkdir arc-api-quickstart
  cd arc-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `arc-api-quickstart` and initializes a Node.js project within it.

### 3. Make your first request

Install Axios, a popular HTTP client, to make API requests:

<CodeGroup>
  ```shell npm
  npm install axios
  ```

  ```shell yarn
  yarn add axios
  ```
</CodeGroup>

Create an `index.js` file in your project directory and paste the following code:

<CodeGroup>
  ```javascript index.js
  const axios = require('axios');

  const url = 'https://arc-testnet.g.alchemy.com/v2/your-api-key';

  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'eth_blockNumber',
    params: []
  };

  axios.post(url, payload)
    .then(response => {
      console.log('Latest Block:', response.data.result);
    })
    .catch(error => {
      console.error(error);
    });
  ```
</CodeGroup>

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

### 4. Run your script

Execute your script to make a request to the Arc network:

<CodeGroup>
  ```shell shell
  node index.js
  ```
</CodeGroup>

You should see the latest block information from Arc's network outputted to your console:

<CodeGroup>
  ```shell shell
  Latest Block: 0x...
  ```
</CodeGroup>

## Next steps

You've made your first request to the Arc network. You can now explore the various JSON-RPC methods available on Arc and start building your dApps on this platform.