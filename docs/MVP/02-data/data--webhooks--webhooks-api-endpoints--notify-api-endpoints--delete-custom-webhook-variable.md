# Delete a Variable

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-custom-webhook-variable.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-custom-webhook-variable.md)

# Delete a Variable

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

DELETE https://dashboard.alchemy.com/api/graphql/variables/{variable}

This endpoint allows you to delete a Custom Webhook variable

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-custom-webhook-variable

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
curl --request DELETE \
  --url https://dashboard.alchemy.com/api/graphql/variables/string \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token'
```

### JavaScript

```javascript
const options = {method: 'DELETE', headers: {'X-Alchemy-Token': 'your-X-Alchemy-Token'}};

> 📄 **This content also appears in [Create a Variable](02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--create-custom-webhook-variable.md)** — see there for full details.

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/graphql/variables/string"

headers = {"X-Alchemy-Token": "your-X-Alchemy-Token"}

response = requests.delete(url, headers=headers)

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

	req, _ := http.NewRequest("DELETE", url, nil)

	req.Header.Add("X-Alchemy-Token", "your-X-Alchemy-Token")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.delete("https://dashboard.alchemy.com/api/graphql/variables/string")
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
var response = await client.DeleteAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /graphql/variables/{variable}
method: DELETE
operation:
  summary: Delete a Variable
  description: This endpoint allows you to delete a Custom Webhook variable
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
  responses:
    '201':
      description: OK- Successful deletion of a Custom Webhook variable
    '400':
      description: Bad Request- The server cannot understand the request.
    '404':
      description: Not found- The requested resource could not be found
    '500':
      description: Internal Server Error- Try again
  operationId: delete-custom-webhook-variable
```
