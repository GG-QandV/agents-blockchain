# Transactions By Wallet (Beta)

> Source: [https://www.alchemy.com/docs/data/beta-apis/beta-api-endpoints/beta-api-endpoints/get-transaction-history-by-address.md](https://www.alchemy.com/docs/data/beta-apis/beta-api-endpoints/beta-api-endpoints/get-transaction-history-by-address.md)

# Transactions By Wallet (Beta)

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/v1/{apiKey}/transactions/history/by-address

Fetches all historical transactions (internal & external) for multiple wallet addresses and networks. (Currently limited to Ethereum and Base with a limit of 1 address) Returns a list of transaction objects with metadata and log information.

<Warning title="To be deprecated">
Please use `alchemy_getAssetTransfers` instead.

While you can continue using this endpoint for now, it is scheduled for removal. The recommended approach is to migrate to `alchemy_getAssetTransfers`, which has coverage for more chains and provides more complete and consistent results.

</Warning>


Reference: https://www.alchemy.com/docs/data/beta-apis/beta-api-endpoints/beta-api-endpoints/get-transaction-history-by-address

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address \
  --header 'Content-Type: application/json' \
  --data '{
  "addresses": [
    {
      "address": "0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152",
      "networks": [
        "eth-mainnet"
      ]
    }
  ],
  "before": "string",
  "after": "string",
  "limit": 25,
  "pageKey": "string"
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    addresses: [
      {
        address: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152',
        networks: ['eth-mainnet']
      }
    ],
    before: 'string',
    after: 'string',
    limit: 25,
    pageKey: 'string'
  })
};

fetch('https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address"

payload = {
    "addresses": [
        {
            "address": "0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152",
            "networks": ["eth-mainnet"]
        }
    ],
    "before": "string",
    "after": "string",
    "limit": 25,
    "pageKey": "string"
}
headers = {"Content-Type": "application/json"}

response = requests.post(url, json=payload, headers=headers)

print(response.text)
```

### Go

```go
package main

import (
	"fmt"
	"strings"
	"net/http"
	"io"
)

func main() {

	url := "https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address"

	payload := strings.NewReader("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"before\": \"string\",\n  \"after\": \"string\",\n  \"limit\": 25,\n  \"pageKey\": \"string\"\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Content-Type", "application/json")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address")
  .header("Content-Type", "application/json")
  .body("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"before\": \"string\",\n  \"after\": \"string\",\n  \"limit\": 25,\n  \"pageKey\": \"string\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/v1/docs-demo/transactions/history/by-address");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"before\": \"string\",\n  \"after\": \"string\",\n  \"limit\": 25,\n  \"pageKey\": \"string\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/transactions/history/by-address
method: POST
operation:
  summary: Transactions By Wallet (Beta)
  description: |
    Fetches all historical transactions (internal & external) for multiple wallet addresses and networks. (Currently limited to Ethereum and Base with a limit of 1 address) Returns a list of transaction objects with metadata and log information.

    <Warning title="To be deprecated">
    Please use `alchemy_getAssetTransfers` instead.

    While you can continue using this endpoint for now, it is scheduled for removal. The recommended approach is to migrate to `alchemy_getAssetTransfers`, which has coverage for more chains and provides more complete and consistent results.

    </Warning>
  tags:
    - Beta API Endpoints
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          properties:
            addresses:
              type: array
              description: |
                Array of address and networks pairs (limit 1 pairs, max 2 networks). Networks should match network enums.
              items:
                type: object
                properties:
                  address:
                    type: string
                    description: Wallet address.
                    example: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152'
                    default: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152'
                  networks:
                    type: array
                    default:
                      - eth-mainnet
                      - base-mainnet
                    items:
                      type: string
                      default: eth-mainnet
                    description: (In BETA and only accepts ETH & BASE mainnets). Network identifier (e.g., eth-mainnet). Find more network enums [here](https://dashboard.alchemy.com/chains)
                required:
                  - address
                  - networks
            before:
              type: string
              description: The cursor that points to the previous set of results.
            after:
              type: string
              description: The cursor that points to the end of the current set of results.
            limit:
              type: integer
              description: 'Sets the maximum number of items per page (Max: 50)'
              default: 25
            pageKey:
              type: string
          required:
            - addresses
  responses:
    '200':
      description: Successful response!
      content:
        application/json:
          schema:
            type: object
            properties:
              before:
                type: string
                description: The cursor that points to the previous set of results.
              after:
                type: string
                description: The cursor that points to the end of the current set of results.
              totalCount:
                type: integer
                description: Total count of the response items.
              transactions:
                type: array
                description: List of transactions by address.
                items:
                  type: object
                  properties:
                    network:
                      type: string
                      description: Network associated with an individual transaction
                    hash:
                      type: string
                      description: Transaction hash
                    timeStamp:
                      type: string
                      description: (ISO 8601)  Timestamp of transaction mining / confirmation
                    blockNumber:
                      type: integer
                      description: Block number of transaction mining / confirmation
                    blockHash:
                      type: string
                      description: Block hash of transaction mining / confirmation
                    nonce:
                      type: integer
                      description: Transaction nonce
                    transactionIndex:
                      type: integer
                      description: Position of transaction within a block
                    fromAddress:
                      type: string
                      description: From address of transaction (hex string).
                    toAddress:
                      type: string
                      description: To address of transaction (hex string). null if contract creation.
                    contractAddress:
                      type: string
                      description: 20 Bytes - The contract address created, if the transaction was a contract creation, otherwise null
                    value:
                      type: string
                      description: (uint8) Value of native token value moved within the external transaction
                    cumulativeGasUsed:
                      type: string
                      description: The total amount of gas used when this transaction was executed in the block.
                    effectiveGasPrice:
                      type: string
                      description: Gas price parameter
                    gasUsed:
                      type: string
                      description: The amount of gas used by this specific transaction alone
                    logs:
                      type: array
                      description: Array of log objects, which this transaction generated
                      items:
                        type: object
                        properties:
                          contractAddress:
                            type: string
                            description: 20 Bytes - contract address from which this log originated.
                          logIndex:
                            type: string
                            description: Integer of the log index position in the block. null when its pending log.
                          data:
                            type: string
                            description: Contains one or more 32 Bytes non-indexed arguments of the log.
                          removed:
                            type: boolean
                            description: true when the log was removed, due to a chain reorganization. false if its a valid log.
                          topics:
                            type: array
                            items:
                              type: string
                            description: Array of zero to four 32 Bytes DATA of indexed log arguments
                    internalTxns:
                      type: array
                      description: Array of internal transaction objects, which this transaction generated
                      items:
                        type: object
                        properties:
                          type:
                            type: string
                            description: CALL or CREATE
                          fromAddress:
                            type: string
                            description: 20 Bytes - address of the sender
                          toAddress:
                            type: string
                            description: 20 Bytes - address of the receiver. null when its a contract creation transaction
                          value:
                            type: string
                            description: amount of value for transfer (in hex)
                          gas:
                            type: string
                            description: amount of gas provided for the call (in hex)
                          gasUsed:
                            type: string
                            description: amount of gas used during the call (in hex)
                          input:
                            type: string
                            description: call data
                          output:
                            type: string
                            description: return data
                          error:
                            type: string
                            description: error, if any
                          revertReason:
                            type: string
                            description: solidity revert reason, if any
            required:
              - transactions
    '400':
      description: 'Bad Request: Invalid input (e.g., malformed JSON).'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
    '429':
      description: 'Too Many Requests: Rate limit exceeded.'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
  operationId: get-transaction-history-by-address
```
