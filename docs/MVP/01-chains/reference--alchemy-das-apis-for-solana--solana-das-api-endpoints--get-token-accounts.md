# getTokenAccounts

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-token-accounts.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-token-accounts.md)

# getTokenAccounts

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns token accounts based on the specified filters. At least one of `ownerAddress` or `mintAddress` is required.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-token-accounts

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ownerAddress | string | Yes | The owner (wallet) address whose token accounts you want to list. Either `ownerAddress` or `mintAddress` must be provided; `ownerAddress` is the common case (passing a wallet pubkey, getting back its token holdings).  |
| mintAddress | string | No | The mint address to filter token accounts by. Pass this INSTEAD OF `ownerAddress` when you want to list every token account for a given mint, or alongside `ownerAddress` to narrow a wallet's holdings to a specific mint. Leave empty if you only want results scoped by owner.  |
| page | integer | No | The page of results to return (1-indexed). Use either page-based pagination (`page`) or cursor-based pagination (`cursor`/`before`/`after`), not both. |
| limit | integer | No | The maximum number of token accounts to retrieve. |
| cursor | string | No | Cursor for pagination. Returned in the previous response. |
| before | string | No | Returns results before the specified cursor. |
| after | string | No | Returns results after the specified cursor. |
| options | object | No | Optional response shaping flags. The server also accepts `displayOptions` as an alias of this field, but only one of `options` or `displayOptions` may be present per request. |

## Result

**Token accounts** (object): Returns token accounts matching the specified criteria.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getTokenAccounts",
  "params": [
    "86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY",
    100
  ],
  "id": 1
}
```

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://solana-mainnet.g.alchemy.com/v2/docs-demo \
  --header 'Content-Type: application/json' \
  --data '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenAccounts",
  "params": [
    "86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY",
    "string",
    1,
    100,
    "string",
    "string",
    "string",
    {
      "showZeroBalance": false
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
    jsonrpc: '2.0',
    id: 1,
    method: 'getTokenAccounts',
    params: [
      '86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY',
      'string',
      1,
      100,
      'string',
      'string',
      'string',
      {showZeroBalance: false}
    ]
  })
};

fetch('https://solana-mainnet.g.alchemy.com/v2/docs-demo', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://solana-mainnet.g.alchemy.com/v2/docs-demo"

payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getTokenAccounts",
    "params": ["86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY", "string", 1, 100, "string", "string", "string", { "showZeroBalance": False }]
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

	url := "https://solana-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getTokenAccounts\",\n  \"params\": [\n    \"86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY\",\n    \"string\",\n    1,\n    100,\n    \"string\",\n    \"string\",\n    \"string\",\n    {\n      \"showZeroBalance\": false\n    }\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://solana-mainnet.g.alchemy.com/v2/docs-demo")
  .header("Content-Type", "application/json")
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getTokenAccounts\",\n  \"params\": [\n    \"86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY\",\n    \"string\",\n    1,\n    100,\n    \"string\",\n    \"string\",\n    \"string\",\n    {\n      \"showZeroBalance\": false\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getTokenAccounts\",\n  \"params\": [\n    \"86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY\",\n    \"string\",\n    1,\n    100,\n    \"string\",\n    \"string\",\n    \"string\",\n    {\n      \"showZeroBalance\": false\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getTokenAccounts
description: Returns token accounts based on the specified filters. At least one of `ownerAddress` or `mintAddress` is required.
x-compute-units: 160
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: ownerAddress
    required: true
    description: |
      The owner (wallet) address whose token accounts you want to list. Either `ownerAddress` or `mintAddress` must be provided; `ownerAddress` is the common case (passing a wallet pubkey, getting back its token holdings).
    schema:
      title: Pubkey
      type: string
      description: Base-58 encoded public key.
  - name: mintAddress
    required: false
    description: |
      The mint address to filter token accounts by. Pass this INSTEAD OF `ownerAddress` when you want to list every token account for a given mint, or alongside `ownerAddress` to narrow a wallet's holdings to a specific mint. Leave empty if you only want results scoped by owner.
    schema:
      title: Pubkey
      type: string
      description: Base-58 encoded public key.
  - name: page
    required: false
    description: The page of results to return (1-indexed). Use either page-based pagination (`page`) or cursor-based pagination (`cursor`/`before`/`after`), not both.
    schema:
      type: integer
      minimum: 1
      default: 1
  - name: limit
    required: false
    description: The maximum number of token accounts to retrieve.
    schema:
      type: integer
      minimum: 1
      maximum: 1000
      default: 100
  - name: cursor
    required: false
    description: Cursor for pagination. Returned in the previous response.
    schema:
      type: string
  - name: before
    required: false
    description: Returns results before the specified cursor.
    schema:
      type: string
  - name: after
    required: false
    description: Returns results after the specified cursor.
    schema:
      type: string
  - name: options
    required: false
    description: Optional response shaping flags. The server also accepts `displayOptions` as an alias of this field, but only one of `options` or `displayOptions` may be present per request.
    schema:
      type: object
      properties:
        showZeroBalance:
          type: boolean
          default: false
          description: If true, include token accounts with a zero token balance.
examples:
  - name: getTokenAccounts example
    params:
      - name: ownerAddress
        value: 86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY
      - name: limit
        value: 100
result:
  name: Token accounts
  description: Returns token accounts matching the specified criteria.
  schema:
    title: Token Accounts List
    type: object
    properties:
      total:
        type: integer
        description: Total number of token accounts.
      limit:
        type: integer
        description: Number of token accounts returned.
      cursor:
        type: string
        nullable: true
        description: Cursor for pagination.
      token_accounts:
        type: array
        description: Array of token accounts.
        items:
          title: Token Account
          type: object
          properties:
            pubkey:
              title: Pubkey
              type: string
              description: The account Pubkey as a base-58 encoded string.
            account:
              title: Account Information
              type: object
              properties:
                lamports:
                  type: integer
                  description: Number of lamports assigned to this account.
                owner:
                  title: Pubkey
                  type: string
                  description: Program owner of this account.
                data:
                  title: Account Data
                  type: array
                  description: Account data in the specified encoding format.
                  items:
                    type: string
                executable:
                  type: boolean
                  description: Indicates if the account contains a program.
                rentEpoch:
                  type: integer
                  description: The epoch at which this account will next owe rent.
                size:
                  type: integer
                  description: The data size of the account.
```
