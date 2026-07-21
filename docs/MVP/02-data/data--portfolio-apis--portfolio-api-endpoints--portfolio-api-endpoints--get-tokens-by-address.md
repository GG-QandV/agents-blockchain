# Tokens By Wallet

> Source: [https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-tokens-by-address.md](https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-tokens-by-address.md)

# Tokens By Wallet

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/v1/{apiKey}/assets/tokens/by-address

Fetches fungible tokens (native, ERC-20 and SPL) for multiple wallet addresses and networks. Returns a list of tokens with balances, prices, and metadata for each wallet/network combination. This endpoint is supported on Ethereum, Solana, and 30+ EVM chains. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).


Reference: https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-tokens-by-address

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address \
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
  "withMetadata": true,
  "withPrices": true,
  "includeNativeTokens": true,
  "includeErc20Tokens": true
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
    withMetadata: true,
    withPrices: true,
    includeNativeTokens: true,
    includeErc20Tokens: true
  })
};

fetch('https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address"

payload = {
    "addresses": [
        {
            "address": "0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152",
            "networks": ["eth-mainnet"]
        }
    ],
    "withMetadata": True,
    "withPrices": True,
    "includeNativeTokens": True,
    "includeErc20Tokens": True
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

	url := "https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address"

	payload := strings.NewReader("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"withMetadata\": true,\n  \"withPrices\": true,\n  \"includeNativeTokens\": true,\n  \"includeErc20Tokens\": true\n}")

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address")
  .header("Content-Type", "application/json")
  .body("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"withMetadata\": true,\n  \"withPrices\": true,\n  \"includeNativeTokens\": true,\n  \"includeErc20Tokens\": true\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/v1/docs-demo/assets/tokens/by-address");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ]\n    }\n  ],\n  \"withMetadata\": true,\n  \"withPrices\": true,\n  \"includeNativeTokens\": true,\n  \"includeErc20Tokens\": true\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/assets/tokens/by-address
method: POST
operation:
  summary: Tokens By Wallet
  description: |
    Fetches fungible tokens (native, ERC-20 and SPL) for multiple wallet addresses and networks. Returns a list of tokens with balances, prices, and metadata for each wallet/network combination. This endpoint is supported on Ethereum, Solana, and 30+ EVM chains. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
  tags:
    - Portfolio API Endpoints
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
                Array of wallet addresses and the networks to query them on. Maximum 2 addresses and maximum 5 networks per address.
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
                      - matic-mainnet
                    items:
                      type: string
                      default: eth-mainnet
                    description: Network identifier (e.g., eth-mainnet). Find more network enums [here](https://dashboard.alchemy.com/chains)
                required:
                  - address
                  - networks
            withMetadata:
              description: Boolean - if set to `true`, returns metadata. Setting this to false will reduce payload size and may result in a faster API call.
              type: boolean
              default: true
            withPrices:
              description: Boolean - if set to `true`, returns token prices. Setting this to false will reduce payload size and may result in a faster API call.
              type: boolean
              default: true
            includeNativeTokens:
              type: boolean
              description: Whether to include each chain's native token in the response (e.g. ETH on Ethereum). The native token will have a null contract address.
              example: true
              default: true
            includeErc20Tokens:
              description: Boolean - if set to `true`, returns ERC-20 tokens. Setting this to false will reduce payload size and may result in a faster API call.
              type: boolean
              default: true
          required:
            - addresses
        example:
          addresses:
            - address: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152'
              networks:
                - eth-mainnet
                - base-mainnet
                - matic-mainnet
          withMetadata: true
          withPrices: true
          includeNativeTokens: true
          includeErc20Tokens: false
  responses:
    '200':
      description: Successful response!
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: object
                description: List of tokens by address, with prices and metadata.
                properties:
                  tokens:
                    type: array
                    items:
                      type: object
                      properties:
                        address:
                          type: string
                          description: Wallet address.
                        network:
                          type: string
                          description: Network identifier.
                        tokenAddress:
                          type: string
                          description: Token address.
                        tokenBalance:
                          type: string
                          description: Balance of that particular token.
                        tokenMetadata:
                          type: object
                          properties:
                            decimals:
                              type: integer
                              description: Number of decimals the token uses
                            logo:
                              type: string
                              description: URL of the token's logo image
                            name:
                              type: string
                              description: Token's name
                            symbol:
                              type: string
                              description: Token's symbol
                        tokenPrices:
                          type: array
                          description: List of price information.
                          items:
                            type: object
                            properties:
                              currency:
                                type: string
                                example: usd
                              value:
                                type: string
                                example: '4608.2208671202'
                              lastUpdatedAt:
                                type: string
                                format: date-time
                                example: '2025-08-26T20:17:27Z'
                            required:
                              - currency
                              - value
                              - lastUpdatedAt
                        error:
                          type:
                            - string
                            - 'null'
                          description: Error message if applicable.
                      required:
                        - network
                        - address
                        - tokenAddress
                        - tokenBalance
                  pageKey:
                    type: string
            required:
              - data
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
  operationId: get-tokens-by-address
```
