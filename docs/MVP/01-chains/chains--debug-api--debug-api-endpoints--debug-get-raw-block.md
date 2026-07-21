# debug_getRawBlock

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-block.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-block.md)

# debug_getRawBlock

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Returns an RLP-encoded block.

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-get-raw-block

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| BlockNumberOrTag | string or enum | Yes | The block number or tag (e.g., "latest", "earliest"). |

## Result

**RLP-encoded block** (string): RLP-encoded block as a hexadecimal string

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_getRawBlock",
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
  "result": "0xf90236a09f73691f6dabca4f0a99b05d0a701995506aa311dcaa9ce9833d6f4ca474c162a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794c6e2459991bfe27cca6d86722f35da23a1e4cb97a078103ea8c47231886481d72ec1afae6eeb06c3773ce24a91323d5c9eed69d4cca0008992da2531db404f07b0871dd620a94ba346963e1b1c6dc7b00748e8593a1ea0b6c3890d9604434fc52f722848c84d1770add20cd75bbc28cdedff42940dbb56b90100200800000400000002000e0000000401000000440100000000c0400600000002000801000000040480020840048000000000400000000000000020004220000011002000000000000204000800000010010002000002000000000040a000000000000400020000010885000000000808000000008800001004002010020300005000000010002110410402000000000000000890000008000000000000000000020040000002000000000000810400000040006000004000004080020000000000000022001000000000000840400000000220250000000000080402000420000418000000000000000400040000004080040010200000000000108020020000808332026e8401c9c380833e3c3c846436f93899d883010b05846765746888676f312e32302e32856c696e7578a0112d8f15793e7df7f8dcdb21c891cff78c0d1839cb5b6dcd06116cdbb99536ae88000000000000000008a0cdb97712af6685bb9650d21d609525913293c48adda7c45990926daada335c9b",
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
  "method": "debug_getRawBlock",
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
  body: JSON.stringify({jsonrpc: '2.0', id: 1, method: 'debug_getRawBlock', params: ['string']})
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
    "method": "debug_getRawBlock",
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawBlock\",\n  \"params\": [\n    \"string\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawBlock\",\n  \"params\": [\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_getRawBlock\",\n  \"params\": [\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_getRawBlock
description: Returns an RLP-encoded block.
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
  name: RLP-encoded block
  description: RLP-encoded block as a hexadecimal string
  schema:
    type: string
    description: RLP-encoded block as a hexadecimal string
examples:
  - name: debug_getRawBlock example
    params:
      - name: BlockNumberOrTag
        value: latest
    result:
      name: debug_getRawBlock response
      value: '0xf90236a09f73691f6dabca4f0a99b05d0a701995506aa311dcaa9ce9833d6f4ca474c162a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794c6e2459991bfe27cca6d86722f35da23a1e4cb97a078103ea8c47231886481d72ec1afae6eeb06c3773ce24a91323d5c9eed69d4cca0008992da2531db404f07b0871dd620a94ba346963e1b1c6dc7b00748e8593a1ea0b6c3890d9604434fc52f722848c84d1770add20cd75bbc28cdedff42940dbb56b90100200800000400000002000e0000000401000000440100000000c0400600000002000801000000040480020840048000000000400000000000000020004220000011002000000000000204000800000010010002000002000000000040a000000000000400020000010885000000000808000000008800001004002010020300005000000010002110410402000000000000000890000008000000000000000000020040000002000000000000810400000040006000004000004080020000000000000022001000000000000840400000000220250000000000080402000420000418000000000000000400040000004080040010200000000000108020020000808332026e8401c9c380833e3c3c846436f93899d883010b05846765746888676f312e32302e32856c696e7578a0112d8f15793e7df7f8dcdb21c891cff78c0d1839cb5b6dcd06116cdbb99536ae88000000000000000008a0cdb97712af6685bb9650d21d609525913293c48adda7c45990926daada335c9b'
```
