# isSpamContract

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-spam-contract.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-spam-contract.md)

# isSpamContract

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/isSpamContract

Returns whether a contract is marked as spam or not by Alchemy.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/is-spam-contract

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

> 📄 **This content also appears in [getContractMetadata](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contract-metadata.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/isSpamContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/isSpamContract
method: GET
operation:
  summary: isSpamContract
  description: Returns whether a contract is marked as spam or not by Alchemy.
  tags:
    - NFT API V2 Methods (Older Version)
  servers:
    - url: https://{network}.g.alchemy.com/nft
      variables:
        network:
          enum:
            - eth-mainnet
            - polygon-mainnet
          default: eth-mainnet
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
        default: '0x495f947276749ce646f68ac8c248420045cb7b5e'
      required: true
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: boolean
            description: |
              True - if the queried contract is marked as spam.
              False - if the queried contract is considered valid or if the contract hasn't been evaluated by us yet.
  operationId: isSpamContract
```
