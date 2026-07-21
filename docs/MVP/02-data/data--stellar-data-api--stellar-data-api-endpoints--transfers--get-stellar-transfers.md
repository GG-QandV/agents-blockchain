# Get Stellar transfer history

> Source: [https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/transfers/get-stellar-transfers.md](https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/transfers/get-stellar-transfers.md)

# Get Stellar transfer history

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/stellar/transfers

Returns transfers (native, classic, and/or Soroban) matching the given filters, ordered by ledger. Use `pageKey` to page through results.

Reference: https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/transfers/get-stellar-transfers

## Headers

> 📄 **This content also appears in [Get Stellar account balances](02-data/data--stellar-data-api--stellar-data-api-endpoints--balances--get-stellar-balances.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/stellar/transfers \
  --header 'Authorization: string' \
  --header 'Content-Type: application/json' \
  --data '{
  "network": "mainnet",
  "fromAddress": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
  "toAddress": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
  "transferType": [
    "native"
  ],
  "assetCode": "USDC",
  "assetIssuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
  "contractId": "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
  "fromLedger": 1,
  "toLedger": 1,
  "order": "desc",
  "limit": 1000,
  "pageKey": "string",
  "excludeFailedTransactions": false
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json', Authorization: 'string'},
  body: JSON.stringify({
    network: 'mainnet',
    fromAddress: 'GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7',
    toAddress: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
    transferType: ['native'],
    assetCode: 'USDC',
    assetIssuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
    contractId: 'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75',
    fromLedger: 1,
    toLedger: 1,
    order: 'desc',
    limit: 1000,
    pageKey: 'string',
    excludeFailedTransactions: false
  })
};

fetch('https://api.g.alchemy.com/data/stellar/transfers', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/stellar/transfers"

payload = {
    "network": "mainnet",
    "fromAddress": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
    "toAddress": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    "transferType": ["native"],
    "assetCode": "USDC",
    "assetIssuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    "contractId": "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
    "fromLedger": 1,
    "toLedger": 1,
    "order": "desc",
    "limit": 1000,
    "pageKey": "string",
    "excludeFailedTransactions": False
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

	url := "https://api.g.alchemy.com/data/stellar/transfers"

	payload := strings.NewReader("{\n  \"network\": \"mainnet\",\n  \"fromAddress\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"toAddress\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"transferType\": [\n    \"native\"\n  ],\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"fromLedger\": 1,\n  \"toLedger\": 1,\n  \"order\": \"desc\",\n  \"limit\": 1000,\n  \"pageKey\": \"string\",\n  \"excludeFailedTransactions\": false\n}")

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/stellar/transfers")
  .header("Content-Type", "application/json")
  .header("Authorization", "string")
  .body("{\n  \"network\": \"mainnet\",\n  \"fromAddress\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"toAddress\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"transferType\": [\n    \"native\"\n  ],\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"fromLedger\": 1,\n  \"toLedger\": 1,\n  \"order\": \"desc\",\n  \"limit\": 1000,\n  \"pageKey\": \"string\",\n  \"excludeFailedTransactions\": false\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/stellar/transfers");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("Authorization", "string");
request.AddJsonBody("{\n  \"network\": \"mainnet\",\n  \"fromAddress\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"toAddress\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"transferType\": [\n    \"native\"\n  ],\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"fromLedger\": 1,\n  \"toLedger\": 1,\n  \"order\": \"desc\",\n  \"limit\": 1000,\n  \"pageKey\": \"string\",\n  \"excludeFailedTransactions\": false\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /data/stellar/transfers
method: POST
operation:
  operationId: getStellarTransfers
  tags:
    - Transfers
  summary: Get Stellar transfer history
  description: Returns transfers (native, classic, and/or Soroban) matching the given filters, ordered by ledger. Use `pageKey` to page through results.
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - network
          properties:
            network:
              type: string
              description: The Stellar network to query.
              enum:
                - mainnet
                - testnet
              default: mainnet
              example: mainnet
            fromAddress:
              type: string
              description: Filter to transfers sent from this account (G... address).
              example: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
            toAddress:
              type: string
              description: Filter to transfers received by this account (G... address).
              example: GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN
            transferType:
              type: array
              description: Restrict to these transfer types. Defaults to all types.
              items:
                type: string
                description: Class of asset movement. `native` = XLM, `classic` = SEP-0011 trustline asset, `soroban` = Soroban smart-contract token.
                enum:
                  - native
                  - classic
                  - soroban
            assetCode:
              type: string
              description: Filter to a classic asset code (e.g. "USDC").
              example: USDC
            assetIssuer:
              type: string
              description: Filter to a classic asset issuer account (G... address).
              example: GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN
            contractId:
              type: string
              description: Filter to a Soroban contract id (C... address).
              example: CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75
            fromLedger:
              type: integer
              format: int64
              description: Inclusive lower bound on ledger sequence.
            toLedger:
              type: integer
              format: int64
              description: Inclusive upper bound on ledger sequence.
            order:
              type: string
              description: Sort direction by ledger.
              enum:
                - asc
                - desc
              default: desc
            limit:
              type: integer
              description: Max transfers to return (1–1000).
              minimum: 1
              maximum: 1000
              default: 1000
            pageKey:
              type: string
              description: Opaque pagination cursor from a prior response. Omit for the first page.
            excludeFailedTransactions:
              type: boolean
              description: When true, omit transfers from failed transactions.
        examples:
          byAddress:
            summary: Recent transfers for an account
            value:
              network: mainnet
              fromAddress: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
              limit: 100
              order: desc
  responses:
    '200':
      description: Matching transfers.
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
                  - transfers
                properties:
                  transfers:
                    type: array
                    description: Matching transfers (empty array when none match).
                    items:
                      type: object
                      description: A single Stellar asset movement.
                      properties:
                        ledgerSequence:
                          type: integer
                          format: int64
                          description: Ledger sequence the operation was included in.
                        txHash:
                          type: string
                          description: Transaction hash.
                        opIndex:
                          type: integer
                          description: Index of the operation within the transaction.
                        timestamp:
                          type: string
                          description: ISO-8601 close time of the ledger.
                        transferType:
                          type: string
                          description: Class of asset movement. `native` = XLM, `classic` = SEP-0011 trustline asset, `soroban` = Soroban smart-contract token.
                          enum:
                            - native
                            - classic
                            - soroban
                        assetCode:
                          type: string
                          description: Asset code for native/classic rows (e.g. "XLM", "USDC").
                        assetIssuer:
                          type: string
                          description: Issuer account for classic rows.
                        contractId:
                          type: string
                          description: Soroban contract id for soroban rows.
                        from:
                          type: string
                          description: Sender account.
                        to:
                          type: string
                          description: Recipient account.
                        amount:
                          type: string
                          description: Transfer amount. Native/classic rows use 7-decimal Stellar display units; raw Soroban rows use base-unit integer strings.
                        success:
                          type: boolean
                          description: Whether the enclosing transaction succeeded.
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
