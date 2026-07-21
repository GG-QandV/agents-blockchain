> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Robinhood Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/robinhood-chain-api-quickstart.md](https://www.alchemy.com/docs/reference/robinhood-chain-api-quickstart.md)

# Robinhood Chain API Quickstart

> How to get started building on Robinhood Chain and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

*To use the Robinhood Chain API you'll need to [create a free Alchemy account](https://dashboard.alchemy.com/signup) first!*

## What is Robinhood Chain?

Robinhood Chain is a permissionless Ethereum Layer 2 built on Arbitrum, optimized for tokenized real-world assets including equities and ETFs, enabling 24/7 onchain trading and self-custody.

## What is the Robinhood Chain API?

The Robinhood Chain API lets you interact with the Robinhood Chain network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Getting started

### 1. Choose a package manager (npm or yarn)

Choose between `npm` and `yarn` based on your preference or project requirements.

> 📄 **This content also appears in [Arc API Quickstart](05-tools-resources/reference--arc-api-quickstart.md)** — see there for full details.

  ```shell yarn
  # For yarn, refer to yarn's installation guide
  # https://classic.yarnpkg.com/lang/en/docs/install
  ```
</CodeGroup>

### 2. Set up your project

Run the following commands to create and initialize your project:

<CodeGroup>
  ```shell npm
  mkdir robinhood-chain-api-quickstart
  cd robinhood-chain-api-quickstart
  npm init --yes
  ```

  ```shell yarn
  mkdir robinhood-chain-api-quickstart
  cd robinhood-chain-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `robinhood-chain-api-quickstart` and initializes a Node.js project within it.

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

  const url = 'https://robinhood-testnet.g.alchemy.com/v2/your-api-key';

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

Replace `your-api-key` with your actual Alchemy API key from the [Alchemy Dashboard](https://dashboard.alchemy.com/signup).

### 4. Run your script

Run your script to make a request to the Robinhood Chain network:

<CodeGroup>
  ```shell shell
  node index.js
  ```
</CodeGroup>

You should see the latest block information from Robinhood Chain's network outputted to your console:

<CodeGroup>
  ```shell shell
  Latest Block: 0x...
  ```
</CodeGroup>

## Next steps

You've made your first request to the Robinhood Chain network. You can now explore the various JSON-RPC methods available on Robinhood Chain and start building your dApps.