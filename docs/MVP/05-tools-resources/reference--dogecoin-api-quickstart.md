> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Dogecoin API Quickstart

> Source: [https://www.alchemy.com/docs/reference/dogecoin-api-quickstart.md](https://www.alchemy.com/docs/reference/dogecoin-api-quickstart.md)

# Dogecoin API Quickstart

> Get started building on Dogecoin and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

# Introduction

The Dogecoin API gives you access to the Dogecoin blockchain through a standard set of JSON-RPC methods. With this API, you can retrieve block and transaction data, inspect the mempool, broadcast transactions, and more.

Dogecoin uses a UTXO-based model and exposes its functionality via the JSON-RPC protocol. This quickstart will help you make your first request.

***

## What is the Dogecoin Chain API?

The Dogecoin Chain API allows applications to communicate with a Dogecoin node using the JSON-RPC protocol. Like Bitcoin, Dogecoin relies on a UTXO (Unspent Transaction Output) model, which means balances are tracked by outputs that are explicitly spent or unspent.

Alchemy's Dogecoin API gives developers a consistent interface to query blockchain data, submit transactions, and monitor network activity. This includes:

> 📄 **This content also appears in [Bitcoin API Quickstart](05-tools-resources/reference--bitcoin-api-quickstart.md)** — see there for full details.

If you've worked with Bitcoin or other JSON-RPC-compatible chains, the structure will feel familiar.

***

## Getting started

### 1. Choose a package manager (npm or yarn)

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

### 2. Set up your project

Open your terminal and run the following commands:

<CodeGroup>
  ```text npm
  mkdir dogecoin-api-quickstart
  cd dogecoin-api-quickstart
  npm init --yes
  ```

  ```text yarn
  mkdir dogecoin-api-quickstart
  cd dogecoin-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `dogecoin-api-quickstart` and initializes a Node.js project within it.

### 3. Make your first request

Install Axios, a popular HTTP client, to make API requests:

<CodeGroup>
  ```bash bash
  npm install axios
  # Or with yarn
  # yarn add axios
  ```
</CodeGroup>

Create an `index.js` file in your project directory and paste the following code:

<CodeGroup>
  ```javascript javascript
  const axios = require('axios');

  const url = `https://dogecoin-mainnet.g.alchemy.com/v2/${apiKey}`;

  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'getblockcount',
    params: []
  };

  > 📄 **This content also appears in [Bitcoin Cash API Quickstart](05-tools-resources/reference--bitcoincash-api-quickstart.md)** — see there for full details.

> 📄 **This content also appears in [Bitcoin Cash API Quickstart](05-tools-resources/reference--bitcoincash-api-quickstart.md)** — see there for full details.

### 4. Run your script

To execute your script and make a request to the Dogecoin API, run:

<CodeGroup>
  ```bash bash
  node index.js
  ```
</CodeGroup>

You should see the current block height on Dogecoin printed to your console:

```shell
Current block height: 5421894
```

## Next steps

You've made your first request to the Dogecoin API. With this foundation, you can dive deeper into the [JSON-RPC methods available on Dogecoin](/docs/dogecoin/dogecoin-api-overview) and start building on it.