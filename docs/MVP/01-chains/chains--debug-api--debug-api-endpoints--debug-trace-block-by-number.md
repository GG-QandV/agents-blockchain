# debug_traceBlockByNumber

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-number.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-number.md)

# debug_traceBlockByNumber

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Replays the block that is already present in the database.

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-block-by-number

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| Block number | string or enum | Yes | The block number or tag (e.g., "latest", "earliest"). |
| Tracer | object | No | Tracer object for the call. |

## Result

**Block traces** (object[]): Array of block traces.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_traceBlockByNumber",
  "params": [
    "0xccde12"
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
    "value": "0x0",
    "gas": "0x7fffffffffffadf7",
    "gasUsed": "0x0",
    "input": "0x",
    "output": "0x"
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
  "method": "debug_traceBlockByNumber",
  "params": [
    "0xccde12",
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
    method: 'debug_traceBlockByNumber',
    params: ['0xccde12', {tracer: 'callTracer', tracerConfig: {onlyTopCall: false}}]
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
    "method": "debug_traceBlockByNumber",
    "params": [
        "0xccde12",
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByNumber\",\n  \"params\": [\n    \"0xccde12\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByNumber\",\n  \"params\": [\n    \"0xccde12\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceBlockByNumber\",\n  \"params\": [\n    \"0xccde12\",\n    {\n      \"tracer\": \"callTracer\",\n      \"tracerConfig\": {\n        \"onlyTopCall\": false\n      }\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_traceBlockByNumber
description: Replays the block that is already present in the database.
x-compute-units: 40
x-rate-limit-cus: 1000
params:
  - name: Block number
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
  - name: debug_traceBlockByNumber example
    params:
      - name: Block number
        value: '0xccde12'
    result:
      name: debug_traceBlockByNumber response
      value:
        type: CALL
        from: '0xe088776deabb472ffd2843e330e79c880a5f979e'
        to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1'
        value: '0x0'
        gas: '0x7fffffffffffadf7'
        gasUsed: '0x0'
        input: 0x
        output: 0x
```
