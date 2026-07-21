# Get all webhooks

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/team-webhooks.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/team-webhooks.md)

# Get all webhooks

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://dashboard.alchemy.com/api/team-webhooks

This endpoint allows you to get all webhooks on your team.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/team-webhooks

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url https://dashboard.alchemy.com/api/team-webhooks \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token'
```

### JavaScript

```javascript
const options = {method: 'GET', headers: {'X-Alchemy-Token': 'your-X-Alchemy-Token'}};

fetch('https://dashboard.alchemy.com/api/team-webhooks', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/team-webhooks"

headers = {"X-Alchemy-Token": "your-X-Alchemy-Token"}

response = requests.get(url, headers=headers)

print(response.text)
```

### Go

```go
package main

import (
	"fmt"
	"net/http"
	"io"
)

func main() {

	url := "https://dashboard.alchemy.com/api/team-webhooks"

	req, _ := http.NewRequest("GET", url, nil)

	req.Header.Add("X-Alchemy-Token", "your-X-Alchemy-Token")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://dashboard.alchemy.com/api/team-webhooks")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/team-webhooks");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /team-webhooks
method: GET
operation:
  summary: Get all webhooks
  description: This endpoint allows you to get all webhooks on your team.
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
  responses:
    '200':
      description: Returns list of webhook objects.
      content:
        application/json:
          schema:
            type: array
            items:
              type: object
              properties:
                data:
                  type: array
                  description: List of webhooks for your team.
                  items:
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
  operationId: team-webhooks
```
