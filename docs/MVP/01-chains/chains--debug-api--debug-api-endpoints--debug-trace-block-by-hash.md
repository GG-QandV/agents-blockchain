# debug_traceBlockByHash

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-hash.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-hash.md)

# debug_traceBlockByHash

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Replays the block that is already present in the database.

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-hash

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| Block hash | string | Yes | Block hash for the block to be traced. |
| Tracer | object | No | Tracer object for the call. |

## Result

**Block traces** (object[]): Array of block traces.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_traceBlockByHash",
  "params": [
    "0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e"
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "type": "CALL",
    "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
    "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
    "value": "0xec5162",
    "gas": "0x7df99",
    "gasUsed": "0x34e29",
    "input": "0x00e051479210030000000000000000000000f160594a405d53811d3bc4766596efd80fd545a27000000000000000000000128acb080000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000ff00000000000000000000000000000000008485b36623632ffa5e486008df4d0b6d363defdb00020000000000000000034a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000005f98805a4e8be255a32880fdec7f6728c6568ba000000000000000000000000000000000000000000000003acbfe2488ff5c000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000000f19663f2ca0454accad3e094448ea6f7744388045400000000000000000000128acb08000000000000000000000000dfee68a9adb981cd08699891a11cabe10f25ec44000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000003a6d6cd1833904000000000000000000000000000000000000000000000000000000000001000276a400000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d5f98805a4e8be255a32880fdec7f6728c6568ba0c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000bb80000"
  },
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
  "method": "debug_traceBlockByHash",
  "params": [
    "0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e",
    {
      "tracer": "callTracer",
      "tracerConfig": {
        "onlyTopCall": false
      }
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
    method: 'debug_traceBlockByHash',
    params: [
      '0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e',
      {tracer: 'callTracer', tracerConfig: {onlyTopCall: false}}
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
    "method": "debug_traceBlockByHash",
    "params": [
        "0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e",
        {
            "tracer": "callTracer",
            "tracerConfig": { "onlyTopCall": False }
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByHash\",\n  \"params\": [\n    \"0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByHash\",\n  \"params\": [\n    \"0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByHash\",\n  \"params\": [\n    \"0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_traceBlockByHash
description: Replays the block that is already present in the database.
x-compute-units: 40
x-rate-limit-cus: 1000
params:
  - name: Block hash
    required: true
    description: Block hash for the block to be traced.
    schema:
      title: 32 byte hex value
      type: string
      pattern: ^0x[0-9a-f]{64}$
  - name: Tracer
    required: false
    description: Tracer object for the call.
    schema:
      title: Tracer
      type: object
      properties:
        tracer:
          type: string
          enum:
            - callTracer
            - prestateTracer
        tracerConfig:
          type: object
          properties:
            onlyTopCall:
              type: boolean
result:
  name: Block traces
  description: Array of block traces.
  schema:
    type: array
    description: Array of block traces.
    items:
      type: object
      description: Array of block traces.
      properties:
        type:
          type: string
          description: CALL or CREATE
        from:
          type: string
          description: 20-byte address of the caller
        to:
          type: string
          description: 20-byte address of the recipient. Null when its a contract creation transaction.
        value:
          type: string
          description: Amount of value included in the transfer (in hex)
        gas:
          type: string
          description: Amount of gas provided for the call (in hex)
        gasUsed:
          type: string
          description: Amount of gas used during the call (in hex)
        input:
          type: string
          description: Call data
        output:
          type: string
          description: Return data
        error:
          type: string
          description: Error message, if any.
        revertReason:
          type: string
          description: Solidity revert reason, if any.
        calls:
          type: array
          description: Array of sub-calls made within the transaction.
          items:
            type: object
            description: Array of block traces.
            properties:
              type:
                type: string
                description: CALL or CREATE
              from:
                type: string
                description: 20-byte address of the caller
              to:
                type: string
                description: 20-byte address of the recipient. Null when its a contract creation transaction.
              value:
                type: string
                description: Amount of value included in the transfer (in hex)
              gas:
                type: string
                description: Amount of gas provided for the call (in hex)
              gasUsed:
                type: string
                description: Amount of gas used during the call (in hex)
              input:
                type: string
                description: Call data
              output:
                type: string
                description: Return data
              error:
                type: string
                description: Error message, if any.
              revertReason:
                type: string
                description: Solidity revert reason, if any.
examples:
  - name: debug_traceBlockByHash example
    params:
      - name: Block hash
        value: '0x7bd8357213af34d3fe7f725d9b21187a5a58127e39aac5776fd0594e3391ea6e'
    result:
      name: debug_traceBlockByHash response
      value:
        type: CALL
        from: '0xe088776deabb472ffd2843e330e79c880a5f979e'
        to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1'
        value: '0xec5162'
        gas: '0x7df99'
        gasUsed: '0x34e29'
        input: '0x00e051479210030000000000000000000000f160594a405d53811d3bc4766596efd80fd545a27000000000000000000000128acb080000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009cc54410f805000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d6b175474e89094c44da98b954eedeac495271d0fc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4010001c4ba12222222228d8ba445958a75a0704d566bf2c80000000000000000000052bbbe2900000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000007f150bd6f54c40a34d7c3d5e9f56000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000ff00000000000000000000000000000000008485b36623632ffa5e486008df4d0b6d363defdb00020000000000000000034a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000005f98805a4e8be255a32880fdec7f6728c6568ba000000000000000000000000000000000000000000000003acbfe2488ff5c000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000000f19663f2ca0454accad3e094448ea6f7744388045400000000000000000000128acb08000000000000000000000000dfee68a9adb981cd08699891a11cabe10f25ec44000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000003a6d6cd1833904000000000000000000000000000000000000000000000000000000000001000276a400000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002d5f98805a4e8be255a32880fdec7f6728c6568ba0c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000bb80000'
```
