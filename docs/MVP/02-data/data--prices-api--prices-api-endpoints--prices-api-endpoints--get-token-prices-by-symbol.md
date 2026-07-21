# Token Prices By Symbol

> Source: [https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-symbol.md](https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-symbol.md)

# Token Prices By Symbol

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://api.g.alchemy.com/prices/v1/{apiKey}/tokens/by-symbol

Fetches current prices for multiple tokens using their symbols. Returns a list of token prices, each containing the symbol, prices, and an optional error field.


Reference: https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-symbol

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| symbols | string[] | Yes | Array of token symbols (limit 25). Example: symbols=[ETH,BTC]  |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH"

response = requests.get(url)

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

	url := "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-symbol?symbols=ETH");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/tokens/by-symbol
method: GET
operation:
  summary: Token Prices By Symbol
  description: |
    Fetches current prices for multiple tokens using their symbols. Returns a list of token prices, each containing the symbol, prices, and an optional error field.
  tags:
    - Prices API Endpoints
  parameters:
    - name: apiKey
      in: path
      required: true
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup).
    - in: query
      name: symbols
      required: true
      description: |
        Array of token symbols (limit 25). Example: symbols=[ETH,BTC]
      schema:
        type: array
        minItems: 1
        items:
          type: string
          default: ETH
      style: form
      explode: true
  responses:
    '200':
      description: Successful response, even if some tokens are missing.
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: array
                description: List of token price data.
                items:
                  type: object
                  properties:
                    symbol:
                      type: string
                      description: Token symbol.
                    prices:
                      type: array
                      description: List of price information.
                      items:
                        type: object
                        properties:
                          currency:
                            type: string
                            description: Currency code (e.g., USD).
                          value:
                            type: string
                            description: Price value as a string.
                          lastUpdatedAt:
                            type: string
                            format: date-time
                            description: Time when the price was last updated.
                        required:
                          - currency
                          - value
                          - lastUpdatedAt
                    error:
                      type: string
                      description: Error message if applicable.
                  required:
                    - symbol
                    - prices
                    - error
            required:
              - data
    '400':
      description: 'Bad Request: Malformed request or missing parameters.'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
    '429':
      description: 'Too Many Requests: Rate limit exceeded.'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
  operationId: get-token-prices-by-symbol
```
