# Update a Variable

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-custom-webhook-variable.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-custom-webhook-variable.md)

# Update a Variable

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

PATCH https://dashboard.alchemy.com/api/graphql/variables/{variable}

Add and remove elements within a Custom Webhook variable

<Info>The use of both `add` and `delete` arrays within a single request is currently not supported. Attempting to send both will result in a 400 error. To add and remove elements, make separate API calls for each action.</Info>


Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/update-custom-webhook-variable

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| variable | string | Yes | String denoting a Custom Webhook variable |

## Code Examples

### cURL

```bash
curl --request PATCH \
  --url https://dashboard.alchemy.com/api/graphql/variables/string \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "add": [
    "string"
  ],
  "delete": [
    "string"
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'PATCH',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({add: ['string'], delete: ['string']})
};

> 📄 **This content also appears in [Create a Variable](02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--create-custom-webhook-variable.md)** — see there for full details.

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/graphql/variables/string"

payload = {
    "add": ["string"],
    "delete": ["string"]
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

	url := "https://dashboard.alchemy.com/api/graphql/variables/string"

	payload := strings.NewReader("{\n  \"add\": [\n    \"string\"\n  ],\n  \"delete\": [\n    \"string\"\n  ]\n}")

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
HttpResponse<String> response = Unirest.patch("https://dashboard.alchemy.com/api/graphql/variables/string")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"add\": [\n    \"string\"\n  ],\n  \"delete\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/graphql/variables/string");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"add\": [\n    \"string\"\n  ],\n  \"delete\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PatchAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /graphql/variables/{variable}
method: PATCH
operation:
  summary: Update a Variable
  description: |
    Add and remove elements within a Custom Webhook variable

    <Info>The use of both `add` and `delete` arrays within a single request is currently not supported. Attempting to send both will result in a 400 error. To add and remove elements, make separate API calls for each action.</Info>
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
    - name: variable
      in: path
      required: true
      description: String denoting a Custom Webhook variable
      schema:
        type: string
  requestBody:
    content:
      application/json:
        schema:
          type: object
          properties:
            add:
              type: array
              default: []
              description: Set of elements to be added to a Custom Webhook variable
              items:
                type: string
            delete:
              type: array
              default: []
              description: Set of elements to be deleted from a Custom Webhook variable
              items:
                type: string
  responses:
    '200':
      description: OK- Successful update to Custom Webhook variable
    '400':
      description: Bad Request- The server cannot understand the request.
    '401':
      description: Unauthorized- missing or invalid authentication credentials
    '500':
      description: Internal Server Error- Try again
  operationId: update-custom-webhook-variable
```
