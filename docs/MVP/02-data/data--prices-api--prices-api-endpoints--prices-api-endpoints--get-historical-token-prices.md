# Historical Token Prices

> Source: [https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-historical-token-prices.md](https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-historical-token-prices.md)

# Historical Token Prices

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/prices/v1/{apiKey}/tokens/historical

Provides historical price data for a single token over a time range. You can identify the token by symbol or by network and contract address.


Reference: https://www.alchemy.com/docs/data/prices-api/prices-api-endpoints/prices-api-endpoints/get-historical-token-prices

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical \
  --header 'Content-Type: application/json' \
  --data '{
  "symbol": "ETH",
  "startTime": "2024-01-01T00:00:00Z",
  "endTime": "2024-01-31T23:59:59Z",
  "interval": "1d",
  "withMarketData": true
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    symbol: 'ETH',
    startTime: '2024-01-01T00:00:00Z',
    endTime: '2024-01-31T23:59:59Z',
    interval: '1d',
    withMarketData: true
  })
};

fetch('https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical"

payload = {
    "symbol": "ETH",
    "startTime": "2024-01-01T00:00:00Z",
    "endTime": "2024-01-31T23:59:59Z",
    "interval": "1d",
    "withMarketData": True
}
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

	url := "https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical"

	payload := strings.NewReader("{\n  \"symbol\": \"ETH\",\n  \"startTime\": \"2024-01-01T00:00:00Z\",\n  \"endTime\": \"2024-01-31T23:59:59Z\",\n  \"interval\": \"1d\",\n  \"withMarketData\": true\n}")

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical")
  .header("Content-Type", "application/json")
  .body("{\n  \"symbol\": \"ETH\",\n  \"startTime\": \"2024-01-01T00:00:00Z\",\n  \"endTime\": \"2024-01-31T23:59:59Z\",\n  \"interval\": \"1d\",\n  \"withMarketData\": true\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/prices/v1/docs-demo/tokens/historical");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"symbol\": \"ETH\",\n  \"startTime\": \"2024-01-01T00:00:00Z\",\n  \"endTime\": \"2024-01-31T23:59:59Z\",\n  \"interval\": \"1d\",\n  \"withMarketData\": true\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/tokens/historical
method: POST
operation:
  summary: Historical Token Prices
  description: |
    Provides historical price data for a single token over a time range. You can identify the token by symbol or by network and contract address.
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
          description: |
            Request body for fetching historical token prices. Provide either the token `symbol` or both `network` and `address`, along with the required time range parameters.
          oneOf:
            - required:
                - symbol
                - startTime
                - endTime
              properties:
                symbol:
                  type: string
                  description: Token symbol (e.g., ETH, BTC).
                  example: ETH
                  default: ETH
                startTime:
                  oneOf:
                    - type: string
                      format: date-time
                      description: Start of the time range in ISO 8601 format.
                      default: '2024-01-01T00:00:00Z'
                    - type: number
                      description: Start of the time range as a timestamp in seconds since epoch.
                      default: 1704067200
                  description: Start of the time range.
                  example: '2024-01-01T00:00:00Z'
                endTime:
                  oneOf:
                    - type: string
                      format: date-time
                      description: End of the time range in ISO 8601 format.
                      default: '2024-01-31T23:59:59Z'
                    - type: number
                      description: End of the time range as a timestamp in seconds since epoch.
                      default: 1706745599
                  description: End of the time range.
                  example: '2024-01-31T23:59:59Z'
                interval:
                  type: string
                  description: |
                    Time interval for data points. Max ranges: (5m, 7d), (1h, 30d), (1d, 1yr)
                  enum:
                    - 5m
                    - 1h
                    - 1d
                  default: 1d
                  example: 1d
                withMarketData:
                  type: boolean
                  description: Whether to include market cap and volume for each token
                  example: true
                  default: false
            - required:
                - network
                - address
                - startTime
                - endTime
              properties:
                network:
                  type: string
                  description: Network identifier (e.g., eth-mainnet).
                  example: eth-mainnet
                  default: eth-mainnet
                address:
                  type: string
                  description: Token contract address.
                  example: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                  default: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                startTime:
                  oneOf:
                    - type: string
                      format: date-time
                      description: Start of the time range in ISO 8601 format.
                      default: '2024-01-01T00:00:00Z'
                    - type: number
                      description: Start of the time range as a timestamp since epoch.
                      default: 1704067200
                  description: Start of the time range.
                  example: '2024-01-01T00:00:00Z'
                endTime:
                  oneOf:
                    - type: string
                      format: date-time
                      description: End of the time range in ISO 8601 format.
                      default: '2024-01-31T23:59:59Z'
                    - type: number
                      description: End of the time range as a timestamp since epoch.
                      default: 1706745599
                  description: End of the time range.
                  example: '2024-01-31T23:59:59Z'
                interval:
                  type: string
                  description: |
                    Time interval for data points. Max ranges: (5m, 7d), (1h, 30d), (1d, 1yr)
                  enum:
                    - 5m
                    - 1h
                    - 1d
                  default: 1d
                  example: 1d
                withMarketData:
                  type: boolean
                  description: Whether to include market cap and volume for each token
                  example: true
                  default: false
  responses:
    '200':
      description: Successful response with historical price data.
      content:
        application/json:
          schema:
            type: object
            description: |
              Response containing historical price data. It will either include `symbol` or both `network` and `address` based on the request.
            oneOf:
              - type: object
                properties:
                  symbol:
                    type: string
                    description: Token symbol.
                    example: ETH
                    default: ETH
                  currency:
                    type: string
                    description: Currency identifier.
                    example: usd
                    default: usd
                  data:
                    type: array
                    description: List of historical price data points.
                    items:
                      type: object
                      properties:
                        value:
                          type: string
                          description: Price value as a string.
                          example: '1900.00'
                        timestamp:
                          type: string
                          format: date-time
                          description: Timestamp of the price data point.
                          example: '2024-01-01T00:00:00Z'
                        marketCap:
                          type: string
                          description: Total market capitalization at the timestamp
                          example: '274292310008.21802'
                        totalVolume:
                          type: string
                          description: Volume traded during the defined interval
                          example: '6715146404.608721'
                      required:
                        - value
                        - timestamp
                required:
                  - symbol
                  - currency
                  - data
              - type: object
                properties:
                  network:
                    type: string
                    description: Network identifier.
                    example: eth-mainnet
                    default: eth-mainnet
                  address:
                    type: string
                    description: Token contract address.
                    example: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                    default: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
                  currency:
                    type: string
                    description: Currency identifier.
                    example: usd
                    default: usd
                  data:
                    type: array
                    description: List of historical price data points.
                    items:
                      type: object
                      properties:
                        value:
                          type: string
                          description: Price value as a string.
                          example: '1900.00'
                        timestamp:
                          type: string
                          format: date-time
                          description: Timestamp of the price data point.
                          example: '2024-01-01T00:00:00Z'
                        marketCap:
                          type: string
                          description: Total market capitalization at the timestamp
                          example: '274292310008.21802'
                        totalVolume:
                          type: string
                          description: Volume traded during the defined interval
                          example: '6715146404.608721'
                      required:
                        - value
                        - timestamp
                required:
                  - network
                  - address
                  - currency
                  - data
    '400':
      description: 'Bad Request: Invalid input (e.g., malformed request, missing parameters).'
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
    '404':
      description: 'Not Found: Token not found.'
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
  operationId: get-historical-token-prices
```
