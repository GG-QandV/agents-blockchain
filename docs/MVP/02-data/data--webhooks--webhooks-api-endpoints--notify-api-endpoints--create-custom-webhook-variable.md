# Create a Variable

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-custom-webhook-variable.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-custom-webhook-variable.md)

# Create a Variable

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://dashboard.alchemy.com/api/graphql/variables/{variable}

This endpoint allows you to create a variable that can be inserted into a Custom Webhook GraphQL statement.

If the variable does not exist, it is created with the provided items. If it already exists, the call is additive: existing items are unchanged, new items are appended, and duplicates are ignored (no error is thrown).


Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/create-custom-webhook-variable

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
curl --request POST \
  --url https://dashboard.alchemy.com/api/graphql/variables/string \
  --header 'Content-Type: application/json' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token' \
  --data '{
  "items": [
    "string"
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json', 'X-Alchemy-Token': 'your-X-Alchemy-Token'},
  body: JSON.stringify({items: ['string']})
};

fetch('https://dashboard.alchemy.com/api/graphql/variables/string', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/graphql/variables/string"

payload = { "items": ["string"] }
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

	url := "https://dashboard.alchemy.com/api/graphql/variables/string"

	payload := strings.NewReader("{\n  \"items\": [\n    \"string\"\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://dashboard.alchemy.com/api/graphql/variables/string")
  .header("Content-Type", "application/json")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .body("{\n  \"items\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/graphql/variables/string");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
request.AddJsonBody("{\n  \"items\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /graphql/variables/{variable}
method: POST
operation:
  summary: Create a Variable
  description: |
    This endpoint allows you to create a variable that can be inserted into a Custom Webhook GraphQL statement.

    If the variable does not exist, it is created with the provided items. If it already exists, the call is additive: existing items are unchanged, new items are appended, and duplicates are ignored (no error is thrown).
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
          required:
            - items
          properties:
            items:
              type: array
              description: A variable defined as a set of addresses or byte32 elements. Must be non-empty.
              default: []
              items:
                type: string
  responses:
    '201':
      description: OK- Successful creation of a Custom Webhook variable
    '400':
      description: Bad Request- The server cannot understand the request.
    '500':
      description: Internal Server Error- Try again
  operationId: create-custom-webhook-variable
```
