# getOwnersForToken

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-token.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-token.md)

# getOwnersForToken

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getOwnersForToken

Get the owner(s) for a token.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-token

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | String - Contract address for the token to get the owner for. |
| tokenId | string | Yes | String - The ID of the token. Can be in hex or decimal format. |
| pageKey | string | No | String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results. |
| pageSize | integer | No | Number of owners to be returned per page. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44"

response = requests.get(url)

print(response.text)
```

### Go

```go
package main

import (
	"fmt"
	"net/http"
	"io"
)

func main() {

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForToken?contractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&tokenId=44");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getOwnersForToken
method: GET
operation:
  summary: getOwnersForToken
  description: Get the owner(s) for a token.
  tags:
    - NFT API V2 Methods (Older Version)
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: contractAddress
      description: String - Contract address for the token to get the owner for.
      in: query
      schema:
        type: string
        default: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48'
      required: true
    - name: tokenId
      description: String - The ID of the token. Can be in hex or decimal format.
      in: query
      schema:
        type: string
        default: '44'
      required: true
    - name: pageKey
      description: String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results.
      schema:
        type: string
      in: query
    - name: pageSize
      description: Number of owners to be returned per page.
      schema:
        type: integer
      in: query
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: object
            properties:
              owners:
                type: array
                description: List of all addresses that own the given NFT.
                items:
                  type: string
  operationId: getOwnersForToken
```
