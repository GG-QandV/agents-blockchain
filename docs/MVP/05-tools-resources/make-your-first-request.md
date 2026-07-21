# Make your first Alchemy request

> Source: [https://www.alchemy.com/docs/make-your-first-request.md](https://www.alchemy.com/docs/make-your-first-request.md)

# Make your first Alchemy request

> Send your first JSON-RPC request to Alchemy with cURL.

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

This guide shows you how to send an `eth_gasPrice` request to Ethereum mainnet
with cURL and verify the response.

## Prerequisites

* An Alchemy API key. If you need one, [create an API key](/docs/create-an-api-key).
* A terminal with `curl` installed

<Steps>
  <Step title="Set your API key">
    Export your API key as an environment variable:

    ```bash
    export ALCHEMY_API_KEY="YOUR_API_KEY"
    ```
  </Step>

  <Step title="Send the request">
    Run this command:

    ```bash
    curl "https://eth-mainnet.g.alchemy.com/v2/$ALCHEMY_API_KEY" \
      -X POST \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}'
    ```
  </Step>

  <Step title="Check the response">
    A successful response looks like this:

    ```json
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": "0x09184e72a000"
    }
    ```

    The `result` value is the current gas price in wei, encoded as hex.
  </Step>
</Steps>

## Next steps

* [Set up Alchemy with Viem](/docs/set-up-alchemy-with-viem)
* [Use API keys in HTTP headers](/docs/how-to-use-api-keys-in-http-headers)
* [Review request logs in the Alchemy Dashboard](/docs/alchemy-request-logs)