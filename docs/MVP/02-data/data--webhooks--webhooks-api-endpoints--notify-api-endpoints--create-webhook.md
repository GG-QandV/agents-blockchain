# Create webhook

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-webhook.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-webhook.md)

# Create webhook

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://dashboard.alchemy.com/api/create-webhook

This endpoint allows you to create a webhook.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-webhook

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://dashboard.alchemy.com/api/create-webhook \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "network": "ETH_MAINNET",
  "webhook_type": "GRAPHQL",
  "webhook_url": "string",
  "name": "string",
  "graphql_query": "string",
  "addresses": [
    "string"
  ],
  "nft_filters": [
    {
      "contract_address": "string",
      "token_id": "string"
    }
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({
    network: 'ETH_MAINNET',
    webhook_type: 'GRAPHQL',
    webhook_url: 'string',
    name: 'string',
    graphql_query: 'string',
    addresses: ['string'],
    nft_filters: [{contract_address: 'string', token_id: 'string'}]
  })
};

fetch('https://dashboard.alchemy.com/api/create-webhook', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/create-webhook"

payload = {
    "network": "ETH_MAINNET",
    "webhook_type": "GRAPHQL",
    "webhook_url": "string",
    "name": "string",
    "graphql_query": "string",
    "addresses": ["string"],
    "nft_filters": [
        {
            "contract_address": "string",
            "token_id": "string"
        }
    ]
}
headers = {
    "Content-Type": "application/json",
    "X-Alchemy-Token": "your-X-Alchemy-Token"
}

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

	url := "https://dashboard.alchemy.com/api/create-webhook"

	payload := strings.NewReader("{\n  \"network\": \"ETH_MAINNET\",\n  \"webhook_type\": \"GRAPHQL\",\n  \"webhook_url\": \"string\",\n  \"name\": \"string\",\n  \"graphql_query\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ],\n  \"nft_filters\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("X-Alchemy-Token", "your-X-Alchemy-Token")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.post("https://dashboard.alchemy.com/api/create-webhook")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"network\": \"ETH_MAINNET\",\n  \"webhook_type\": \"GRAPHQL\",\n  \"webhook_url\": \"string\",\n  \"name\": \"string\",\n  \"graphql_query\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ],\n  \"nft_filters\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/create-webhook");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"network\": \"ETH_MAINNET\",\n  \"webhook_type\": \"GRAPHQL\",\n  \"webhook_url\": \"string\",\n  \"name\": \"string\",\n  \"graphql_query\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ],\n  \"nft_filters\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /create-webhook
method: POST
operation:
  summary: Create webhook
  description: This endpoint allows you to create a webhook.
  tags:
    - Notify API Endpoints
  parameters:
    - name: X-Alchemy-Token
      in: header
      required: true
      description: Alchemy Auth token to use the Notify API.
      schema:
        type: string
      example: your-X-Alchemy-Token
  requestBody:
    content:
      application/json:
        schema:
          type: object
          required:
            - network
            - webhook_type
            - webhook_url
          properties:
            network:
              type: string
              description: Network of the webhook
              enum:
                - ETH_MAINNET
                - ETH_SEPOLIA
                - ETH_HOLESKY
                - ARB_MAINNET
                - ARB_SEPOLIA
                - ARBNOVA_MAINNET
                - MATIC_MAINNET
                - MATIC_MUMBAI
                - OPT_MAINNET
                - OPT_GOERLI
                - BASE_MAINNET
                - BASE_SEPOLIA
                - ZKSYNC_MAINNET
                - ZKSYNC_SEPOLIA
                - LINEA_MAINNET
                - LINEA_SEPOLIA
                - GNOSIS_MAINNET
                - GNOSIS_CHIADO
                - FANTOM_MAINNET
                - FANTOM_TESTNET
                - METIS_MAINNET
                - BLAST_MAINNET
                - BLAST_SEPOLIA
                - SHAPE_MAINNET
                - SHAPE_SEPOLIA
                - ZETACHAIN_MAINNET
                - ZETACHAIN_TESTNET
                - WORLDCHAIN_MAINNET
                - WORLDCHAIN_SEPOLIA
                - BNB_MAINNET
                - BNB_TESTNET
                - AVAX_MAINNET
                - AVAX_FUJI
                - SONEIUM_MAINNET
                - SONEIUM_MINATO
                - GEIST_POLTER
                - GEIST_MAINNET
                - STARKNET_MAINNET
                - STARKNET_SEPOLIA
                - STARKNET_GOERLI
                - INK_MAINNET
                - INK_SEPOLIA
                - ROOTSTOCK_MAINNET
                - ROOTSTOCK_TESTNET
                - SCROLL_MAINNET
                - SCROLL_SEPOLIA
                - MONAD_TESTNET
                - SONIC_MAINNET
                - SONIC_TESTNET
                - SETTLUS_SEPTESTNET
                - APECHAIN_MAINNET
                - APECHAIN_CURTIS
              default: ETH_MAINNET
            webhook_type:
              type: string
              description: Type of webhook.
              enum:
                - GRAPHQL
                - ADDRESS_ACTIVITY
                - NFT_ACTIVITY
            webhook_url:
              type: string
              description: URL where requests are sent
            name:
              type: string
              description: Name of the webhook
            graphql_query:
              oneOf:
                - type: string
                  description: A valid, stringified GraphQL query to host on Alchemy. Required for GRAPHQL webhook type.
                - type: object
                  description: Required for GRAPHQL webhook type.
                  required:
                    - query
                  properties:
                    query:
                      type: string
                      description: A valid, stringified GraphQL query to host on Alchemy
                    skip_empty_messages:
                      type: boolean
                      description: If true, skip sending webhooks for empty messages.
            addresses:
              type: array
              description: List of addresses to track. Required for ADDRESS_ACTIVITY webhook type.
              items:
                type: string
            nft_filters:
              type: array
              description: List of NFT filter objects to track. Required for NFT_ACTIVITY webhook type.
              items:
                type: object
                properties:
                  contract_address:
                    type: string
                    description: Contract address for an NFT. If this field and the `token_id` aren't set, all NFT activity updates will be sent.
                  token_id:
                    type: string
                    description: Token ID for an NFT (decimal or "0x" prefixed hex string). Cannot be set if `contract_address` is not provided.
  responses:
    '200':
      description: Returns webhook creation data.
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: object
                properties:
                  id:
                    type: string
                    description: Unique ID for the webhook.
                  network:
                    type: string
                    description: Network of the webhook
                    enum:
                      - ETH_MAINNET
                      - ETH_SEPOLIA
                      - ETH_HOLESKY
                      - ARB_MAINNET
                      - ARB_SEPOLIA
                      - ARBNOVA_MAINNET
                      - MATIC_MAINNET
                      - MATIC_MUMBAI
                      - OPT_MAINNET
                      - OPT_GOERLI
                      - BASE_MAINNET
                      - BASE_SEPOLIA
                      - ZKSYNC_MAINNET
                      - ZKSYNC_SEPOLIA
                      - LINEA_MAINNET
                      - LINEA_SEPOLIA
                      - GNOSIS_MAINNET
                      - GNOSIS_CHIADO
                      - FANTOM_MAINNET
                      - FANTOM_TESTNET
                      - METIS_MAINNET
                      - BLAST_MAINNET
                      - BLAST_SEPOLIA
                      - SHAPE_MAINNET
                      - SHAPE_SEPOLIA
                      - ZETACHAIN_MAINNET
                      - ZETACHAIN_TESTNET
                      - WORLDCHAIN_MAINNET
                      - WORLDCHAIN_SEPOLIA
                      - BNB_MAINNET
                      - BNB_TESTNET
                      - AVAX_MAINNET
                      - AVAX_FUJI
                      - SONEIUM_MAINNET
                      - SONEIUM_MINATO
                      - GEIST_POLTER
                      - GEIST_MAINNET
                      - STARKNET_MAINNET
                      - STARKNET_SEPOLIA
                      - STARKNET_GOERLI
                      - INK_MAINNET
                      - INK_SEPOLIA
                      - ROOTSTOCK_MAINNET
                      - ROOTSTOCK_TESTNET
                      - SCROLL_MAINNET
                      - SCROLL_SEPOLIA
                      - MONAD_TESTNET
                      - SONIC_MAINNET
                      - SONIC_TESTNET
                      - SETTLUS_SEPTESTNET
                      - APECHAIN_MAINNET
                      - APECHAIN_CURTIS
                    default: ETH_MAINNET
                  webhook_type:
                    type: string
                    description: Type of webhook.
                    enum:
                      - GRAPHQL
                      - ADDRESS_ACTIVITY
                      - NFT_ACTIVITY
                  webhook_url:
                    type: string
                    description: URL endpoint where the webhook is sent
                  is_active:
                    type: boolean
                    description: True if the webhook is active, false otherwise.
                  time_created:
                    type: integer
                    description: Timestamp when the webhook was created.
                  version:
                    type: string
                    description: Webhook version (v1 or v2)
                  signing_key:
                    type: string
                    description: Signing key for the webhook.
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: create-webhook
```
