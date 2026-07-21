# Get Variable Elements

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/read-custom-webhook-variable.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/read-custom-webhook-variable.md)

# Get Variable Elements

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://dashboard.alchemy.com/api/graphql/variables/{variable}

This endpoint allows you to read the values within a Custom Webhook variable.
It supports pagination with `limit` and `after` (and optionally `pageKey`) query parameters.


Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/read-custom-webhook-variable

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| variable | string | Yes | String denoting a Custom Webhook variable |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | The maximum number of items to return per page. |
| after | string | No | The cursor that points to the end of the current set of results. |
| pageKey | string | No | Page cursor for the next page. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url https://dashboard.alchemy.com/api/graphql/variables/string \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token'
```

### JavaScript

```javascript
const options = {method: 'GET', headers: {'X-Alchemy-Token': 'your-X-Alchemy-Token'}};

> 📄 **This content also appears in [Create a Variable](02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--create-custom-webhook-variable.md)** — see there for full details.

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/graphql/variables/string"

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

	url := "https://dashboard.alchemy.com/api/graphql/variables/string"

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
HttpResponse<String> response = Unirest.get("https://dashboard.alchemy.com/api/graphql/variables/string")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/graphql/variables/string");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /graphql/variables/{variable}
method: GET
operation:
  summary: Get Variable Elements
  description: |
    This endpoint allows you to read the values within a Custom Webhook variable.
    It supports pagination with `limit` and `after` (and optionally `pageKey`) query parameters.
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
      description: OK- Successful query of Custom Webhook variable
      content:
        application/json:
          schema:
            type: array
            items:
              type: object
              properties:
                data:
                  type: array
                  description: List of addresses associated with the webhook.
                  items:
                    type: string
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
                      description: Total number of addresses.
    '400':
      description: Bad Request- The server cannot understand the request.
    '404':
      description: Not found- The requested resource could not be found
    '500':
      description: Internal Server Error- Try again
  operationId: read-custom-webhook-variable
```
