# debug_getRawReceipts

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-receipts.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-receipts.md)

# debug_getRawReceipts

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Returns an array of EIP-2718 binary-encoded receipts.

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-receipts

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| BlockNumberOrTag | string or enum | Yes | The block number or tag (e.g., "latest", "earliest"). |

## Result

**RLP-encoded receipts** (string[]): Array of EIP-2718 binary-encoded receipts as hexadecimal strings.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_getRawReceipts",
  "params": [
    "latest"
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": [
    "0xf901a60182c70eb9010000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002000000000000000000000008000000000000000000000000000000000040000000001000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000100000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000002000000000100000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000020000000000000000f89df89b947753cfad258efbc52a9a1452e42ffbce9be486cbf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000828d0386c1122e565f07dd28c7d1340ed5b3315a000000000000000000000000021849e99c31e3113a489d7eb0fd4d8c0edbe47afa00000000000000000000000000000000000000000000000000000000029b92700",
    "0xf901a70183018e1cb9010000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002000000000000000000000008000000000000000000000000000000000040000000001000000000000000000000000000000000000000000000000010000000000000000000000000000000008000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000002000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000020000000000000000f89df89b947753cfad258efbc52a9a1452e42ffbce9be486cbf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000828d0386c1122e565f07dd28c7d1340ed5b3315a000000000000000000000000069cda9d6cc6ce05982d0b4fdf9480f2991f39b5aa00000000000000000000000000000000000000000000000000000000029b92700"
  ],
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
  "method": "debug_getRawReceipts",
  "params": [
    "string"
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({jsonrpc: '2.0', id: 1, method: 'debug_getRawReceipts', params: ['string']})
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
    "method": "debug_getRawReceipts",
    "params": ["string"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawReceipts\",\n  \"params\": [\n    \"string\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawReceipts\",\n  \"params\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawReceipts\",\n  \"params\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_getRawReceipts
description: Returns an array of EIP-2718 binary-encoded receipts.
x-compute-units: 40
x-rate-limit-cus: 1000
params:
  - name: BlockNumberOrTag
    required: true
    description: The block number or tag (e.g., "latest", "earliest").
    schema:
      oneOf:
        - title: hex encoded bytes
          type: string
          pattern: ^0x[0-9a-f]*$
        - title: block tag
          type: string
          enum:
            - latest
            - earliest
            - pending
result:
  name: RLP-encoded receipts
  description: Array of EIP-2718 binary-encoded receipts as hexadecimal strings.
  schema:
    type: array
    items:
      type: string
examples:
  - name: debug_getRawReceipts example
    params:
      - name: BlockNumberOrTag
        value: latest
    result:
      name: debug_getRawReceipts response
      value:
        - '0xf901a60182c70eb9010000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002000000000000000000000008000000000000000000000000000000000040000000001000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000100000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000002000000000100000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000020000000000000000f89df89b947753cfad258efbc52a9a1452e42ffbce9be486cbf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000828d0386c1122e565f07dd28c7d1340ed5b3315a000000000000000000000000021849e99c31e3113a489d7eb0fd4d8c0edbe47afa00000000000000000000000000000000000000000000000000000000029b92700'
        - '0xf901a70183018e1cb9010000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002000000000000000000000008000000000000000000000000000000000040000000001000000000000000000000000000000000000000000000000010000000000000000000000000000000008000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000002000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000020000000000000000f89df89b947753cfad258efbc52a9a1452e42ffbce9be486cbf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000828d0386c1122e565f07dd28c7d1340ed5b3315a000000000000000000000000069cda9d6cc6ce05982d0b4fdf9480f2991f39b5aa00000000000000000000000000000000000000000000000000000000029b92700'
```
