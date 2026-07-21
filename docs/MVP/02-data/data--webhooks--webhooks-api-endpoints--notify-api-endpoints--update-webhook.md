# Update webhook

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook.md)

# Update webhook

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

PUT https://dashboard.alchemy.com/api/update-webhook

Update webhook properties such as name or status.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request PUT \
  --url https://dashboard.alchemy.com/api/update-webhook \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "webhook_id": "string",
  "is_active": false,
  "name": "string"
}'
```

### JavaScript

```javascript
const options = {
  method: 'PUT',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({webhook_id: 'string', is_active: false, name: 'string'})
};

fetch('https://dashboard.alchemy.com/api/update-webhook', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/update-webhook"

payload = {
    "webhook_id": "string",
    "is_active": False,
    "name": "string"
}
headers = {
    "Content-Type": "application/json",
    "X-Alchemy-Token": "your-X-Alchemy-Token"
}

response = requests.put(url, json=payload, headers=headers)

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

	url := "https://dashboard.alchemy.com/api/update-webhook"

	payload := strings.NewReader("{\n  \"webhook_id\": \"string\",\n  \"is_active\": false,\n  \"name\": \"string\"\n}")

	req, _ := http.NewRequest("PUT", url, payload)

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
HttpResponse<String> response = Unirest.put("https://dashboard.alchemy.com/api/update-webhook")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"webhook_id\": \"string\",\n  \"is_active\": false,\n  \"name\": \"string\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/update-webhook");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"webhook_id\": \"string\",\n  \"is_active\": false,\n  \"name\": \"string\"\n}", false);
var response = await client.PutAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /update-webhook
method: PUT
operation:
  summary: Update webhook
  description: Update webhook properties such as name or status.
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
            - webhook_id
          properties:
            webhook_id:
              type: string
              description: ID of the webhook
            is_active:
              type: boolean
              description: |
                Set webhook status (true for active, false for inactive)
            name:
              type: string
              description: Update the webhook name
  responses:
    '200':
      description: Returns updated webhook data.
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                allOf:
                  - type: object
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
                  - type: object
                    properties:
                      signing_key:
                        type: string
                        description: Signing key for the webhook.
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: update-webhook
```
