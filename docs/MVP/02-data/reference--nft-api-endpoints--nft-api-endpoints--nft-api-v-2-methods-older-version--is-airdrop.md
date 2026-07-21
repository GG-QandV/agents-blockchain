# isAirdrop

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-airdrop.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-airdrop.md)

# isAirdrop

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/isAirdrop

Returns whether a token is marked as an airdrop or not. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-airdrop

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

> 📄 **This content also appears in [computeRarity](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--compute-rarity.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isAirdrop?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/isAirdrop
method: GET
operation:
  summary: isAirdrop
  description: Returns whether a token is marked as an airdrop or not. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address.
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
        default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
      required: true
    - name: tokenId
      description: String - The ID of the token. Can be in hex or decimal format.
      in: query
      schema:
        type: string
        default: '44'
      required: true
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: boolean
            description: |
              True - if the queried token is marked as an airdrop.
              False - if the queried token is not marked as an airdrop.
  operationId: isAirdrop
```
