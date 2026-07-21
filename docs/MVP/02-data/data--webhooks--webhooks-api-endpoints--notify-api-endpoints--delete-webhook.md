# Delete webhook

> Source: [https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-webhook.md](https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-webhook.md)

# Delete webhook

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

DELETE https://dashboard.alchemy.com/api/delete-webhook

Allows you to delete a webhook.

Reference: https://www.alchemy.com/docs/data/webhooks/webhooks-api-endpoints/notify-api-endpoints/delete-webhook

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| X-Alchemy-Token | string | Yes | Alchemy Auth token to use the Notify API. |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| webhook_id | string | Yes | ID of the address activity webhook. |

## Code Examples

### cURL

```bash
curl --request DELETE \
  --url 'https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string' \
  --header 'X-Alchemy-Token: your-X-Alchemy-Token'
```

### JavaScript

```javascript
const options = {method: 'DELETE', headers: {'X-Alchemy-Token': 'your-X-Alchemy-Token'}};

fetch('https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string"

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

	url := "https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string"

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
HttpResponse<String> response = Unirest.delete("https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string")
  .header("X-Alchemy-Token", "your-X-Alchemy-Token")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://dashboard.alchemy.com/api/delete-webhook?webhook_id=string");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("X-Alchemy-Token", "your-X-Alchemy-Token");
var response = await client.DeleteAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /delete-webhook
method: DELETE
operation:
  summary: Delete webhook
  description: Allows you to delete a webhook.
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
  responses:
    '200':
      description: Returns empty object.
      content:
        application/json:
          schema:
            type: object
    '400':
      description: Bad Request- The server cannot understand the request.
  operationId: delete-webhook
```
