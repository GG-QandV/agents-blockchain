# Replace webhook addresses

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/replace-webhook-addresses.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/replace-webhook-addresses.md)

# Replace webhook addresses

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

PUT https://dashboard.alchemy.com/api/update-webhook-addresses

Replace entire list of addresses tracked in a given webhook.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/replace-webhook-addresses

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Code Examples

### cURL

```bash
curl --request PUT \
  --url https://dashboard.alchemy.com/api/update-webhook-addresses \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "webhook_id": "string",
  "addresses": [
    "string"
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'PUT',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({webhook_id: 'string', addresses: ['string']})
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
    "addresses": ["string"]
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

	url := "https://dashboard.alchemy.com/api/update-webhook-addresses"

	payload := strings.NewReader("{\n  \"webhook_id\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ]\n}")

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
HttpResponse<String> response = Unirest.put("https://dashboard.alchemy.com/api/update-webhook-addresses")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"webhook_id\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/update-webhook-addresses");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"webhook_id\": \"string\",\n  \"addresses\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PutAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /update-webhook-addresses
method: PUT
operation:
  summary: Replace webhook addresses
  description: Replace entire list of addresses tracked in a given webhook.
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
            - addresses
          properties:
            webhook_id:
              type: string
              description: ID of the address activity webhook.
            addresses:
              type: array
              description: New list of addresses to track (replaces any existing addresses).
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
  operationId: replace-webhook-addresses
```
