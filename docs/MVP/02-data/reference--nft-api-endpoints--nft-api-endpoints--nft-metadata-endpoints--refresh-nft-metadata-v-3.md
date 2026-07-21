# Refresh NFT Metadata

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/refresh-nft-metadata-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/refresh-nft-metadata-v-3.md)

# Refresh NFT Metadata

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/refreshNftMetadata

Submits a request for Alchemy to refresh the cached metadata of a specific NFT token.

<Note>Please note that this endpoint is only supported on Ethereum (Mainnet & Sepolia), Polygon (Mainnet, Mumbai & Amoy), Arbitrum One (mainnet), Optimism (mainnet) & Base (mainnet). For other chains, you could use the `getNFTMetadata` endpoint with the `refreshCache` parameter set to `true` to refresh the metadata!</Note>


Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/refresh-nft-metadata-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata \
  --header 'Content-Type: application/json' \
  --data '{
  "contractAddress": "0xe785E82358879F061BC3dcAC6f0444462D4b5330",
  "tokenId": "44"
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({contractAddress: '0xe785E82358879F061BC3dcAC6f0444462D4b5330', tokenId: '44'})
};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata"

payload = {
    "contractAddress": "0xe785E82358879F061BC3dcAC6f0444462D4b5330",
    "tokenId": "44"
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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata"

	payload := strings.NewReader("{\n  \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n  \"tokenId\": \"44\"\n}")

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
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata")
  .header("Content-Type", "application/json")
  .body("{\n  \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n  \"tokenId\": \"44\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/refreshNftMetadata");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n  \"tokenId\": \"44\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/refreshNftMetadata
method: POST
operation:
  summary: Refresh NFT Metadata
  description: |
    Submits a request for Alchemy to refresh the cached metadata of a specific NFT token.

    <Note>Please note that this endpoint is only supported on Ethereum (Mainnet & Sepolia), Polygon (Mainnet, Mumbai & Amoy), Arbitrum One (mainnet), Optimism (mainnet) & Base (mainnet). For other chains, you could use the `getNFTMetadata` endpoint with the `refreshCache` parameter set to `true` to refresh the metadata!</Note>
  tags:
    - NFT Metadata Endpoints
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
  requestBody:
    content:
      application/json:
        schema:
          type: object
          required:
            - contractAddress
            - tokenId
          properties:
            contractAddress:
              type: string
              description: Contract address of the token you want to refresh.
              default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
            tokenId:
              type: string
              description: Token ID of the token you want to refresh. Must belong to the contract address.
              default: '44'
  responses:
    '200':
      description: Returns the status of the refresh request along with the estimated time to complete.
      content:
        application/json:
          schema:
            type: object
            properties:
              status:
                type: string
                description: If the token is successfully queued for ingestion the value will be "Queued".
              estimatedMsToRefresh:
                type: string
                description: Estimated time until the metadata refresh is complete for this token.
          examples:
            byDefault:
              summary: Successful Response
              value:
                status: Queued
                estimatedMsToRefresh: 10000
  operationId: refreshNftMetadata-v3
```
