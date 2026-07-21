# Add and remove webhook addresses

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-addresses.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-addresses.md)

# Add and remove webhook addresses

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

PATCH https://dashboard.alchemy.com/api/update-webhook-addresses

Add or remove addresses from a specific webhook.
*This webhook endpoint is idempotent, meaning that identical requests can be made once or several times with the same effect.*


Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-webhook-addresses

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request PATCH \
  --url https://dashboard.alchemy.com/api/update-webhook-addresses \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "webhook_id": "string",
  "addresses_to_add": [
    "string"
  ],
  "addresses_to_remove": [
    "string"
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
    addresses_to_add: ['string'],
    addresses_to_remove: ['string']
  })
};

fetch('https://dashboard.alchemy.com/api/update-webhook-addresses', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/update-webhook-addresses"

payload = {
    "webhook_id": "string",
    "addresses_to_add": ["string"],
    "addresses_to_remove": ["string"]
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

	url := "https://dashboard.alchemy.com/api/update-webhook-addresses"

	payload := strings.NewReader("{\n  \"webhook_id\": \"string\",\n  \"addresses_to_add\": [\n    \"string\"\n  ],\n  \"addresses_to_remove\": [\n    \"string\"\n  ]\n}")

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
HttpResponse<String> response = Unirest.patch("https://dashboard.alchemy.com/api/update-webhook-addresses")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"webhook_id\": \"string\",\n  \"addresses_to_add\": [\n    \"string\"\n  ],\n  \"addresses_to_remove\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/update-webhook-addresses");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"webhook_id\": \"string\",\n  \"addresses_to_add\": [\n    \"string\"\n  ],\n  \"addresses_to_remove\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PatchAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /update-webhook-addresses
method: PATCH
operation:
  summary: Add and remove webhook addresses
  description: |
    Add or remove addresses from a specific webhook.
    *This webhook endpoint is idempotent, meaning that identical requests can be made once or several times with the same effect.*
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
            - addresses_to_add
            - addresses_to_remove
          properties:
            webhook_id:
              type: string
              description: ID of the address activity webhook
            addresses_to_add:
              type: array
              default: []
              description: List of addresses to add **(empty array if none)**.
              items:
                type: string
            addresses_to_remove:
              type: array
              default: []
              description: List of addresses to remove **(empty array if none)**.
              items:
                type: string
  responses:
    '200':
      description: Returns empty object.
      content:
        application/json:
          schema:
            type: object
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: update-webhook-addresses
```
