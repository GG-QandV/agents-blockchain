# getOwnersForCollection

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-collection.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-collection.md)

# getOwnersForCollection

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getOwnersForCollection

Gets all owners for a given NFT contract.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-owners-for-collection

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |
| withTokenBalances | boolean | No | Boolean - If set to `true` the query will include the token balances per token id for each owner. `false` by default. |
| pageKey | string | No | String - used for collections with >50,000 owners. `pageKey` field can be passed back as request parameter to get the next page of results. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getOwnersForCollection?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getOwnersForCollection
method: GET
operation:
  summary: getOwnersForCollection
  description: Gets all owners for a given NFT contract.
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
      description: String - Contract address for the NFT contract (ERC721 and ERC1155 supported).
      in: query
      schema:
        type: string
        default: '0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734'
      required: true
    - name: withTokenBalances
      description: Boolean - If set to `true` the query will include the token balances per token id for each owner. `false` by default.
      in: query
      schema:
        type: boolean
        default: false
    - description: String - used for collections with >50,000 owners. `pageKey` field can be passed back as request parameter to get the next page of results.
      name: pageKey
      schema:
        type: string
      in: query
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: array
            description: List of all addresses that own the given NFT.
            items:
              type: string
  operationId: getOwnersForCollection
```
