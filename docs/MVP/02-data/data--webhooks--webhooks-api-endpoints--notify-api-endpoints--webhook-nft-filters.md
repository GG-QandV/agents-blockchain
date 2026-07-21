# Get all webhook NFT filters

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/webhook-nft-filters.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/webhook-nft-filters.md)

# Get all webhook NFT filters

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://dashboard.alchemy.com/api/webhook-nft-filters

Paginated endpoint to list all of the NFT filter objects a given webhook is subscribed to.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/webhook-nft-filters

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| webhook_id | string | Yes | ID of the address activity webhook. |
| limit | integer | No | The maximum number of items to return per page. |
| after | string | No | The cursor that points to the end of the current set of results. |
| pageKey | string | No | Page cursor for the next page. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token'
```

### JavaScript

```javascript
const options = {method: 'GET', headers: {'X-Alchemy-Token': 'your-X-Alchemy-Token'}};

fetch('https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string"

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

	url := "https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string"

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
HttpResponse<String> response = Unirest.get("https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/webhook-nft-filters?webhook_id=string");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /webhook-nft-filters
method: GET
operation:
  summary: Get all webhook NFT filters
  description: Paginated endpoint to list all of the NFT filter objects a given webhook is subscribed to.
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
    - name: webhook_id
      in: query
      required: true
      description: ID of the address activity webhook.
      schema:
        type: string
    - name: limit
      in: query
      required: false
      description: The maximum number of items to return per page.
      schema:
        type: integer
        default: 100
    - name: after
      in: query
      required: false
      description: The cursor that points to the end of the current set of results.
      schema:
        type: string
        description: The cursor that points to the end of the current set of results.
    - name: pageKey
      in: query
      required: false
      description: Page cursor for the next page.
      schema:
        type: string
        default: '0'
  responses:
    '200':
      description: Returns a list of nft filter objects.
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: array
                description: List of NFT filter objects.
                items:
                  type: object
                  properties:
                    contract_address:
                      type: string
                      description: Contract address for an NFT. If this field and the `token_id` aren't set, all NFT activity updates will be sent.
                    token_id:
                      type: string
                      description: Token ID for an NFT (decimal or "0x" prefixed hex string). Cannot be set if `contract_address` is not provided.
              pagination:
                type: object
                description: Pagination information.
                properties:
                  cursors:
                    type: object
                    description: Pagination cursors.
                    properties:
                      after:
                        type: string
                        description: The cursor that points to the end of the current set of results.
                  total_count:
                    type: integer
                    description: Total number of NFT filters.
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: webhook-nft-filters
```
