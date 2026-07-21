# getNFTSales

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-sales.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-sales.md)

# getNFTSales

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getNFTSales

Gets NFT sales that have happened through on-chain marketplaces

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-sales

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| fromBlock | string | No | String - The block number to start fetching NFT sales data from. Allowed values are decimal and hex integers, and "latest". Defaults to "0". |
| toBlock | string | No | String - The block number to start fetching NFT sales data from. Allowed values are decimal and hex integers, and "latest". Defaults to "latest". |
| order | enum | No | Enum - Whether to return the results ascending from startBlock or descending from startBlock. Defaults to descending (false). |
| marketplace | enum | No | Enum - The name of the NFT marketplace to filter sales by. The endpoint currently supports "seaport", "wyvern", "looksrare", "x2y2", "blur", and "cryptopunks". Defaults to returning sales from all supported marketplaces. |
| contractAddress | string | No | String - The contract address of a NFT collection to filter sales by. Defaults to returning all NFT contracts. |
| tokenId | string | No | String - The token ID of an NFT within the collection specified by contractAddress to filter sales by. Defaults to returning all token IDs. |
| buyerAddress | string | No | String - The address of the NFT buyer to filter sales by. Defaults to returning sales involving any buyer. |
| sellerAddress | string | No | String - The address of the NFT seller to filter sales by. Defaults to returning sales involving any seller. |
| taker | enum | No | Enum - Filter by whether the buyer or seller was the taker in the NFT trade. Allowed filter values are "BUYER" and "SELLER". Defaults to returning both buyer and seller taker trades. |
| limit | integer | No | Integer - The maximum number of NFT sales to return. Maximum and default values are 1000. |
| pageKey | string | No | String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTSales");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getNFTSales
method: GET
operation:
  summary: getNFTSales
  description: Gets NFT sales that have happened through on-chain marketplaces
  tags:
    - NFT API V2 Methods (Older Version)
  servers:
    - url: https://{network}.g.alchemy.com/nft
      variables:
        network:
          enum:
            - eth-mainnet
            - polygon-mainnet
            - opt-mainnet
          default: eth-mainnet
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: fromBlock
      description: String - The block number to start fetching NFT sales data from. Allowed values are decimal and hex integers, and "latest". Defaults to "0".
      in: query
      schema:
        type: string
        default: '0'
    - name: toBlock
      description: String - The block number to start fetching NFT sales data from. Allowed values are decimal and hex integers, and "latest". Defaults to "latest".
      in: query
      schema:
        type: string
        default: latest
    - name: order
      description: Enum - Whether to return the results ascending from startBlock or descending from startBlock. Defaults to descending (false).
      in: query
      schema:
        type: string
        enum:
          - asc
          - desc
        default: asc
    - name: marketplace
      description: Enum - The name of the NFT marketplace to filter sales by. The endpoint currently supports "seaport", "wyvern", "looksrare", "x2y2", "blur", and "cryptopunks". Defaults to returning sales from all supported marketplaces.
      in: query
      schema:
        type: string
        enum:
          - seaport
          - looksrare
          - x2y2
          - wyvern
          - blur
          - cryptopunks
      required: false
    - description: String - The contract address of a NFT collection to filter sales by. Defaults to returning all NFT contracts.
      name: contractAddress
      in: query
      schema:
        type: string
      required: false
    - description: String - The token ID of an NFT within the collection specified by contractAddress to filter sales by. Defaults to returning all token IDs.
      name: tokenId
      in: query
      schema:
        type: string
        default: '44'
      required: false
    - name: buyerAddress
      description: String - The address of the NFT buyer to filter sales by. Defaults to returning sales involving any buyer.
      in: query
      schema:
        type: string
    - name: sellerAddress
      description: String - The address of the NFT seller to filter sales by. Defaults to returning sales involving any seller.
      in: query
      schema:
        type: string
    - name: taker
      description: Enum - Filter by whether the buyer or seller was the taker in the NFT trade. Allowed filter values are "BUYER" and "SELLER". Defaults to returning both buyer and seller taker trades.
      in: query
      schema:
        type: string
        enum:
          - BUYER
          - SELLER
      required: false
    - description: Integer - The maximum number of NFT sales to return. Maximum and default values are 1000.
      name: limit
      in: query
      schema:
        type: integer
    - name: pageKey
      description: String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results.
      schema:
        type: string
      in: query
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: object
            properties:
              nftSales:
                description: List of NFT sales that match the query
                type: array
                items:
                  type: object
                  properties:
                    marketplace:
                      type: string
                      description: String - The marketplace the sale took place on.
                    contractAddress:
                      type: string
                      description: String - The contract address of the collection the NFT belongs to.
                    tokenId:
                      type: string
                      description: String - The decimal token ID of the NFT being sold.
                    quantity:
                      type: string
                      description: Integer - The number of tokens sold in the sale as a decimal integer string.
                    buyerAddress:
                      type: string
                      description: String - The address of the buyer in the NFT sale.
                    sellerAddress:
                      type: string
                      description: String - The address of the seller in the NFT sale.
                    taker:
                      type: string
                      description: String - Whether the price taker in the trade was the buyer or the seller.
                      enum:
                        - BUYER
                        - SELLER
                    sellerFee:
                      type: object
                      description: The payment from buyer to the seller
                      properties:
                        amount:
                          type: string
                          description: String - The amount of the payment from the buyer to seller as a decimal integer string.
                        symbol:
                          type: string
                          description: String - The symbol of the token used for the payment.
                        decimals:
                          type: integer
                          description: Integer - The number of decimals of the token used for the payment.
                    protocolFee:
                      type: object
                      description: The payment from buyer to the NFT marketplace protocol
                      properties:
                        amount:
                          type: string
                          description: String - The amount of the payment to the marketplace as a decimal integer string.
                        symbol:
                          type: string
                          description: String - The symbol of the token used for the payment.
                        decimals:
                          type: integer
                          description: Integer - The number of decimals of the token used for the payment.
                    royaltyFee:
                      type: object
                      description: The payment from buyer to the royalty address of the NFT collection
                      properties:
                        amount:
                          type: string
                          description: String - The amount of the payment to the royalty collector as a decimal integer string.
                        symbol:
                          type: string
                          description: String - The symbol of the token used for the payment.
                        decimals:
                          type: integer
                          description: Integer - The number of decimals of the token used for the payment.
                    blockNumber:
                      type: integer
                      description: Integer - The block number the NFT sale took place in.
                    logIndex:
                      type: integer
                      description: Integer - The log number of the sale event emitted within the block.
                    bundleIndex:
                      type: integer
                      description: Integer - The index of the token within the bundle of NFTs sold in the sale.
                    transactionHash:
                      type: string
                      description: String - The transaction hash of the transaction containing the sale.
              pageKey:
                type: string
                description: String - The page key to use to fetch the next page of results. Returns null if there are no more results.
          examples:
            nftSales_response:
              summary: Response (with pagination)
              value:
                nftSales:
                  - marketplace: seaport
                    contractAddress: '0x49cf6f5d44e70224e2e23fdcdd2c053f30ada28b'
                    tokenId: '13749'
                    quantity: '1'
                    buyerAddress: '0x78f6c2458b53d0735208992c693bb2b2dafebb52'
                    sellerAddress: '0x558a18f94cabdea4e47c5965384f457d8e870419'
                    taker: BUYER
                    sellerFee:
                      amount: '11100000000000000000'
                      symbol: ETH
                      decimals: 18
                    protocolFee:
                      amount: '300000000000000000'
                      symbol: ETH
                      decimals: 18
                    royaltyFee:
                      amount: '600000000000000000'
                      symbol: ETH
                      decimals: 18
                    blockNumber: 15000002
                    logIndex: 130
                    bundleIndex: 0
                    transactionHash: '0xecfa1b29c9016bd2556fde637c6b48484eeb14f273af54c49317e3856ab7cb16'
                  - marketplace: looksrare
                    contractAddress: '0x34d85c9cdeb23fa97cb08333b511ac86e1c4e258'
                    tokenId: '75417'
                    quantity: '1'
                    buyerAddress: '0xb3aa9923489bc2bfec323bf05346acd4afbc92a0'
                    sellerAddress: '0x206ccba024c236dced07c35b4e9eb0bade7ef166'
                    taker: BUYER
                    sellerFee:
                      amount: '2222700000000000000'
                      symbol: WETH
                      decimals: 18
                    protocolFee:
                      amount: '47800000000000000'
                      symbol: WETH
                      decimals: 18
                    royaltyFee:
                      amount: '119500000000000000'
                      symbol: WETH
                      decimals: 18
                    blockNumber: 15000002
                    logIndex: 197
                    bundleIndex: 0
                    transactionHash: '0x4c23163e4f855e143e573776bc6129bee370dff6ce760e71553fc93201b292e2'
                pageKey: MTUwMDAwNzgsODcsMA
                validAt:
                  blockNumber: 17091500
                  blockHash: '0x2a34a65c4e0cd7fdf187d6a497214ad2bee255d2d3501868a6b8c09b4d1261bd'
                  blockTimestamp: '2023-04-21T01:25:59Z'
  operationId: getNFTSales
```
