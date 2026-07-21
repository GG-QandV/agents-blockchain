# Get Stellar NFT holdings

> Source: [https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/nf-ts/get-stellar-nfts.md](https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/nf-ts/get-stellar-nfts.md)

# Get Stellar NFT holdings

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/stellar/nfts

Returns NFTs held by an account. Filtering is exclusive: set `contractId` to restrict to a Soroban contract, or `assetCode` + `assetIssuer` to restrict to a classic asset. Setting both is rejected.

Reference: https://www.alchemy.com/docs/data/stellar-data-api/stellar-data-api-endpoints/nf-ts/get-stellar-nfts

## Headers

> 📄 **This content also appears in [Get Stellar account balances](02-data/data--stellar-data-api--stellar-data-api-endpoints--balances--get-stellar-balances.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/stellar/nfts \
  --header 'Authorization: string' \
  --header 'Content-Type: application/json' \
  --data '{
  "network": "mainnet",
  "address": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
  "contractId": "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
  "assetCode": "USDC",
  "assetIssuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
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
    contractId: 'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75',
    assetCode: 'USDC',
    assetIssuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
    limit: 1,
    pageKey: 'string'
  })
};

fetch('https://api.g.alchemy.com/data/stellar/nfts', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/stellar/nfts"

payload = {
    "network": "mainnet",
    "address": "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
    "contractId": "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
    "assetCode": "USDC",
    "assetIssuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
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

	url := "https://api.g.alchemy.com/data/stellar/nfts"

	payload := strings.NewReader("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}")

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/stellar/nfts")
  .header("Content-Type", "application/json")
  .header("Authorization", "string")
  .body("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/stellar/nfts");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddHeader("Authorization", "string");
request.AddJsonBody("{\n  \"network\": \"mainnet\",\n  \"address\": \"GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7\",\n  \"contractId\": \"CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75\",\n  \"assetCode\": \"USDC\",\n  \"assetIssuer\": \"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\n  \"limit\": 1,\n  \"pageKey\": \"string\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /data/stellar/nfts
method: POST
operation:
  operationId: getStellarNfts
  tags:
    - NFTs
  summary: Get Stellar NFT holdings
  description: 'Returns NFTs held by an account. Filtering is exclusive: set `contractId` to restrict to a Soroban contract, or `assetCode` + `assetIssuer` to restrict to a classic asset. Setting both is rejected.'
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
              description: The account (G... address) to fetch NFT holdings for.
              example: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
            contractId:
              type: string
              description: Restrict to Soroban holdings of this contract id. Mutually exclusive with assetCode/assetIssuer.
              example: CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75
            assetCode:
              type: string
              description: Restrict to this classic asset code. Must be paired with assetIssuer. Mutually exclusive with contractId.
              example: USDC
            assetIssuer:
              type: string
              description: Classic asset issuer account, paired with assetCode.
              example: GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN
            limit:
              type: integer
              description: Max holdings per page.
              minimum: 1
              maximum: 1000
            pageKey:
              type: string
              description: Opaque pagination cursor from a prior response. Omit for the first page.
        examples:
          allNfts:
            summary: All NFTs held by an account
            value:
              network: mainnet
              address: GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7
              limit: 100
  responses:
    '200':
      description: NFT holdings.
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
                  - nfts
                properties:
                  nfts:
                    type: array
                    description: NFT holdings (empty array when none match).
                    items:
                      type: object
                      description: A single NFT held by an account. The `type` field discriminates which fields are populated.
                      properties:
                        type:
                          type: string
                          description: '`soroban` (contractId + tokenId set) or `classic` (assetCode + assetIssuer set).'
                          enum:
                            - soroban
                            - classic
                        contractId:
                          type: string
                          description: Soroban contract id (soroban holdings).
                        tokenId:
                          type: string
                          description: Token id within the Soroban contract (soroban holdings).
                        assetCode:
                          type: string
                          description: Classic asset code (classic holdings).
                        assetIssuer:
                          type: string
                          description: Classic asset issuer account (classic holdings).
                  asOfLedger:
                    type: integer
                    format: int64
                    description: Ledger sequence the holdings were computed as of.
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
