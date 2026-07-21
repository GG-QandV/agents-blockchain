# alchemy_getTokenAllowance

> Source: [https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-allowance.md](https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-allowance.md)

# alchemy_getTokenAllowance

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Returns the token allowance by spender for a given owner.

Reference: https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-allowance

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tokenAllowanceRequest | object | Yes | An object specifying the token contract address, the owner address, and the spender address.  |

## Result

**Allowance** (string): The amount that the spender is allowed to withdraw from the owner.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_getTokenAllowance",
  "params": [
    {
      "contract": "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270",
      "owner": "0xf1a726210550c306a9964b251cbcd3fa5ecb275d",
      "spender": "0xdef1c0ded9bec7f1a1670819833240f027b25eff"
    }
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": "0",
  "id": 1
}
```

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/v2/docs-demo \
  --header 'Content-Type: application/json' \
  --data '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "alchemy_getTokenAllowance",
  "params": [
    {
      "contract": "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270",
      "owner": "0xf1a726210550c306a9964b251cbcd3fa5ecb275d",
      "spender": "0xdef1c0ded9bec7f1a1670819833240f027b25eff"
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
    method: 'alchemy_getTokenAllowance',
    params: [
      {
        contract: '0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270',
        owner: '0xf1a726210550c306a9964b251cbcd3fa5ecb275d',
        spender: '0xdef1c0ded9bec7f1a1670819833240f027b25eff'
      }
    ]
  })
};

fetch('https://eth-mainnet.g.alchemy.com/v2/docs-demo', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "alchemy_getTokenAllowance",
    "params": [
        {
            "contract": "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270",
            "owner": "0xf1a726210550c306a9964b251cbcd3fa5ecb275d",
            "spender": "0xdef1c0ded9bec7f1a1670819833240f027b25eff"
        }
    ]
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

	url := "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenAllowance\",\n  \"params\": [\n    {\n      \"contract\": \"0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270\",\n      \"owner\": \"0xf1a726210550c306a9964b251cbcd3fa5ecb275d\",\n      \"spender\": \"0xdef1c0ded9bec7f1a1670819833240f027b25eff\"\n    }\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/v2/docs-demo")
  .header("Content-Type", "application/json")
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenAllowance\",\n  \"params\": [\n    {\n      \"contract\": \"0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270\",\n      \"owner\": \"0xf1a726210550c306a9964b251cbcd3fa5ecb275d\",\n      \"spender\": \"0xdef1c0ded9bec7f1a1670819833240f027b25eff\"\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenAllowance\",\n  \"params\": [\n    {\n      \"contract\": \"0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270\",\n      \"owner\": \"0xf1a726210550c306a9964b251cbcd3fa5ecb275d\",\n      \"spender\": \"0xdef1c0ded9bec7f1a1670819833240f027b25eff\"\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_getTokenAllowance
description: Returns the token allowance by spender for a given owner.
x-compute-units: 20
params:
  - name: tokenAllowanceRequest
    required: true
    description: |
      An object specifying the token contract address, the owner address, and the spender address.
    schema:
      type: object
      required:
        - contract
        - owner
        - spender
      properties:
        contract:
          description: 20-byte token contract address.
          title: hex encoded address
          type: string
          pattern: ^0x[0-9a-fA-F]{40}$
        owner:
          description: 20-byte address of the owner.
          title: hex encoded address
          type: string
          pattern: ^0x[0-9a-fA-F]{40}$
        spender:
          description: 20-byte address of the spender.
          title: hex encoded address
          type: string
          pattern: ^0x[0-9a-fA-F]{40}$
result:
  name: Allowance
  description: The amount that the spender is allowed to withdraw from the owner.
  schema:
    type: string
    description: A decimal string representing the allowance.
examples:
  - name: Basic allowance example
    params:
      - name: tokenAllowanceRequest
        value:
          contract: '0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270'
          owner: '0xf1a726210550c306a9964b251cbcd3fa5ecb275d'
          spender: '0xdef1c0ded9bec7f1a1670819833240f027b25eff'
    result:
      name: Allowance
      value: '0'
```
