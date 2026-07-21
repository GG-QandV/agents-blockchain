# Update webhook NFT filters

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-nft-filters.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-nft-filters.md)

# Update webhook NFT filters

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

PATCH https://dashboard.alchemy.com/api/update-webhook-nft-filters

Add and remove webhook NFT filters

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-nft-filters

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request PATCH \
  --url https://dashboard.alchemy.com/api/update-webhook-nft-filters \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "webhook_id": "string",
  "nft_filters_to_add": [
    {
      "contract_address": "string",
      "token_id": "string"
    }
  ],
  "nft_filters_to_remove": [
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
  method: 'PATCH',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({
    webhook_id: 'string',
    nft_filters_to_add: [{contract_address: 'string', token_id: 'string'}],
    nft_filters_to_remove: [{contract_address: 'string', token_id: 'string'}]
  })
};

fetch('https://dashboard.alchemy.com/api/update-webhook-nft-filters', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/update-webhook-nft-filters"

payload = {
    "webhook_id": "string",
    "nft_filters_to_add": [
        {
            "contract_address": "string",
            "token_id": "string"
        }
    ],
    "nft_filters_to_remove": [
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

response = requests.patch(url, json=payload, headers=headers)

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

	url := "https://dashboard.alchemy.com/api/update-webhook-nft-filters"

	payload := strings.NewReader("{\n  \"webhook_id\": \"string\",\n  \"nft_filters_to_add\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ],\n  \"nft_filters_to_remove\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}")

	req, _ := http.NewRequest("PATCH", url, payload)

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
HttpResponse<String> response = Unirest.patch("https://dashboard.alchemy.com/api/update-webhook-nft-filters")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"webhook_id\": \"string\",\n  \"nft_filters_to_add\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ],\n  \"nft_filters_to_remove\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/update-webhook-nft-filters");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"webhook_id\": \"string\",\n  \"nft_filters_to_add\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ],\n  \"nft_filters_to_remove\": [\n    {\n      \"contract_address\": \"string\",\n      \"token_id\": \"string\"\n    }\n  ]\n}", false);
var response = await client.PatchAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /update-webhook-nft-filters
method: PATCH
operation:
  summary: Update webhook NFT filters
  description: Add and remove webhook NFT filters
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
              description: ID of the address activity webhook
            nft_filters_to_add:
              type: array
              default: []
              description: List of NFT filters to add (empty array if none).
              items:
                type: object
                properties:
                  contract_address:
                    type: string
                    description: Contract address for an NFT. If this field and the `token_id` aren't set, all NFT activity updates will be sent.
                  token_id:
                    type: string
                    description: Token ID for an NFT (decimal or "0x" prefixed hex string). Cannot be set if `contract_address` is not provided.
            nft_filters_to_remove:
              type: array
              default: []
              description: List of NFT filters to remove (empty array if none).
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
      description: Returns empty object.
      content:
        application/json:
          schema:
            type: object
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: update-webhook-nft-filters
```
