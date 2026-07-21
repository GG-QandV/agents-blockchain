# Token Prices By Address

> Source: [https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-address.md](https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-address.md)

# Token Prices By Address

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/prices/v1/{apiKey}/tokens/by-address

Fetches current prices for multiple tokens using network and address pairs. Returns a list of token prices, each containing the network, address, prices, and an optional error field.


Reference: https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-token-prices-by-address

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address \
  --header 'Content-Type: application/json' \
  --data '{
  "addresses": [
    {
      "network": "eth-mainnet",
      "address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
    }
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    addresses: [
      {network: 'eth-mainnet', address: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'}
    ]
  })
};

fetch('https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address"

payload = { "addresses": [
        {
            "network": "eth-mainnet",
            "address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        }
    ] }
headers = {"Content-Type": "application/json"}

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

	url := "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address"

	payload := strings.NewReader("{\n  \"addresses\": [\n    {\n      \"network\": \"eth-mainnet\",\n      \"address\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    }\n  ]\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Content-Type", "application/json")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address")
  .header("Content-Type", "application/json")
  .body("{\n  \"addresses\": [\n    {\n      \"network\": \"eth-mainnet\",\n      \"address\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/by-address");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"addresses\": [\n    {\n      \"network\": \"eth-mainnet\",\n      \"address\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/tokens/by-address
method: POST
operation:
  summary: Token Prices By Address
  description: |
    Fetches current prices for multiple tokens using network and address pairs. Returns a list of token prices, each containing the network, address, prices, and an optional error field.
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
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          properties:
            addresses:
              type: array
              minItems: 1
              description: |
                Array of token network and address pairs (limit 25 addresses, max 3 networks). Networks should match network enums.
              items:
                type: object
                properties:
                  network:
                    type: string
                    example: eth-mainnet
                    default: eth-mainnet
                    description: Network identifier (e.g., eth-mainnet). Find more network enums [here](https://dashboard.alchemy.com/chains)
                  address:
                    type: string
                    description: Token contract address.
                    example: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                    default: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                required:
                  - network
                  - address
              maxItems: 25
          required:
            - addresses
  responses:
    '200':
      description: Successful response, even if some prices are missing.
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: array
                description: List of token prices by address.
                items:
                  type: object
                  properties:
                    network:
                      type: string
                      description: Network identifier.
                    address:
                      type: string
                      description: Token contract address.
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
                    - network
                    - address
                    - prices
                    - error
            required:
              - data
    '400':
      description: 'Bad Request: Invalid input (e.g., malformed JSON).'
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
  operationId: get-token-prices-by-address
```
