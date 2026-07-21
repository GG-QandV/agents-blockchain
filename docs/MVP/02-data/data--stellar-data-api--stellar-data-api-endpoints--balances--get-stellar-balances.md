# Get Stellar account balances

> Source: [https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/balances/get-stellar-balances.md](https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/balances/get-stellar-balances.md)

# Get Stellar account balances

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/stellar/balances

Returns the unified balance set for an account: native XLM, classic trustline assets, and Soroban contract tokens. Native XLM is emitted only on the first page (when `pageKey` is omitted).

Reference: https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/balances/get-stellar-balances

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| Authorization | string | Yes | Your Alchemy API key, sent as a Bearer token (e.g. `Bearer your-api-key`). |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/stellar/balances \
  --header 'Authorization: string' \
  --header 'Content-Type: application/json' \
  --data '{
  "network": "mainnet",
  "address": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
  "limit": 1,
  "pageKey": "string"
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json', Authorization: 'string'},
  body: JSON.stringify({
    network: 'mainnet',
    address: 'GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7',
    limit: 1,
    pageKey: 'string'
  })
};

fetch('https://api.g.alchemy.com/data/stellar/balances', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/stellar/balances"

payload = {
    "network": "mainnet",
    "address": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
    "limit": 1,
    "pageKey": "string"
}
headers = {
    "Content-Type": "application/json",
    "Authorization": "string"
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

	url := "https://api.g.alchemy.com/data/stellar/balances"

	payload := strings.NewReader("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", "string")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/stellar/balances")
  .header("Content-Type", "application/json")
  .header("Authorization", "string")
  .body("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/stellar/balances");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("Authorization", "string");
request.AddJsonBody("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /data/stellar/balances
method: POST
operation:
  operationId: getStellarBalances
  tags:
    - Balances
  summary: Get Stellar account balances
  description: 'Returns the unified balance set for an account: native XLM, classic trustline assets, and Soroban contract tokens. Native XLM is emitted only on the first page (when `pageKey` is omitted).'
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - network
            - address
          properties:
            network:
              type: string
              description: The Stellar network to query.
              enum:
                - mainnet
                - testnet
              default: mainnet
              example: mainnet
            address:
              type: string
              description: The account (G... address) to fetch balances for.
              example: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
            limit:
              type: integer
              description: Max non-native (classic + soroban) entries per page.
              minimum: 1
              maximum: 1000
            pageKey:
              type: string
              description: Opaque pagination cursor from a prior response. Omit for the first page (which includes native XLM); pass back verbatim to continue.
        examples:
          firstPage:
            summary: First page of balances
            value:
              network: mainnet
              address: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
              limit: 100
  responses:
    '200':
      description: Account balances.
      content:
        application/json:
          schema:
            type: object
            required:
              - data
            properties:
              data:
                type: object
                required:
                  - balances
                properties:
                  balances:
                    type: array
                    items:
                      type: object
                      description: A single balance row. The `type` field discriminates which fields are populated (see field descriptions).
                      properties:
                        type:
                          type: string
                          description: '`native` (XLM), `classic` (trustline asset), or `soroban` (contract token).'
                          enum:
                            - native
                            - classic
                            - soroban
                        assetCode:
                          type: string
                          description: Asset code (native => "XLM"; classic => the code; null for soroban).
                        assetIssuer:
                          type: string
                          description: Issuer account for classic assets.
                        contractId:
                          type: string
                          description: Soroban contract id for soroban balances.
                        balance:
                          type: string
                          description: Balance amount. Classic/native use 7-decimal display units; raw Soroban uses base-unit integer strings.
                        trustlineExists:
                          type: boolean
                          description: Whether a trustline entry exists (native/classic only).
                        authorized:
                          type: boolean
                          description: Whether the trustline is authorized (classic only).
                        limit:
                          type: string
                          description: Trustline limit (classic only).
                  asOfLedger:
                    type: integer
                    format: int64
                    description: Ledger sequence the balances were computed as of.
                  pageKey:
                    type: string
                    description: Cursor for the next page; absent when the result set is exhausted.
    '400':
      description: The request was malformed or failed validation.
      content:
        application/json:
          schema:
            type: object
            description: Standard error payload.
            properties:
              message:
                type: string
                description: Human-readable description of the error.
            required:
              - message
    '401':
      description: Missing or invalid API key.
      content:
        application/json:
          schema:
            type: object
            description: Standard error payload.
            properties:
              message:
                type: string
                description: Human-readable description of the error.
            required:
              - message
```
