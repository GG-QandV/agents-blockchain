# Collections By Owner

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-collections-for-owner-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-collections-for-owner-v-3.md)

# Collections By Owner

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getCollectionsForOwner

Retrieves all NFT collections held by a specified owner address.

<Note>This endpoint is only supported on Ethereum. Use `getContractsForOwner` for support across all other chains we support!</Note>


Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-collections-for-owner-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| owner | string | Yes | String - Address for NFT owner (can be in ENS format for Eth Mainnet). |
| pageKey | string | No | String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results. |
| pageSize | integer | No | Number of NFTs to be returned per page. Defaults to 100. Max is 100. |
| withMetadata | boolean | No | Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`. |
| includeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. Only NFTs that match one or more of these filters will be included in the response. May not be used in conjunction with excludeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |
| excludeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. NFTs that match one or more of these filters will be excluded from the response. May not be used in conjunction with includeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getCollectionsForOwner
method: GET
operation:
  summary: Collections By Owner
  description: |
    Retrieves all NFT collections held by a specified owner address.

    <Note>This endpoint is only supported on Ethereum. Use `getContractsForOwner` for support across all other chains we support!</Note>
  tags:
    - NFT Ownership Endpoints
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: owner
      description: String - Address for NFT owner (can be in ENS format for Eth Mainnet).
      schema:
        type: string
        default: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
      in: query
      required: true
    - name: pageKey
      description: String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results.
      schema:
        type: string
      in: query
    - name: pageSize
      description: Number of NFTs to be returned per page. Defaults to 100. Max is 100.
      schema:
        type: integer
        default: 100
      in: query
    - name: withMetadata
      description: Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`.
      schema:
        type: boolean
        default: true
      in: query
    - name: includeFilters[]
      description: |-
        Array of filters (as ENUMS) that will be applied to the query. Only NFTs that match one or more of these filters will be included in the response. May not be used in conjunction with excludeFilters[]. Filter Options:
          - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.
          - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.
          - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts)
      schema:
        type: array
        items:
          type: string
          enum:
            - SPAM
            - AIRDROPS
          default: SPAM
      in: query
    - name: excludeFilters[]
      description: |-
        Array of filters (as ENUMS) that will be applied to the query. NFTs that match one or more of these filters will be excluded from the response. May not be used in conjunction with includeFilters[]. Filter Options:
          - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.
          - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.
          - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts)
      schema:
        type: array
        items:
          type: string
          enum:
            - SPAM
            - AIRDROPS
          default: SPAM
      in: query
  responses:
    '200':
      description: Returns a list of NFT collections held by the specified owner address.
      content:
        application/json:
          schema:
            type: object
            properties:
              collections:
                type: array
                items:
                  type: object
                  description: Metadata for an NFT collection held by an owner address. Includes general metadata about the collection, as well as information specific to the owner such as the total balance and the token ID of a random NFT for display purposes.
                  properties:
                    name:
                      description: The name of the collection, i.e. "Bored Ape Yacht Club".
                      type: string
                    slug:
                      description: The human-readable string used to identify the collection on OpenSea.
                      type: string
                    floorPrice:
                      type: object
                      description: Floor price data for the collection
                      properties:
                        marketplace:
                          description: The marketplace the floor price is on
                          type: string
                        floorPrice:
                          description: Floor price of the collection on the marketplace
                          type: number
                        priceCurrency:
                          description: The currency of the floor price
                          type: string
                    description:
                      type: string
                      description: 'OpenSea collection description. Note: this value is truncated to 255 characters.'
                    externalUrl:
                      type: string
                      description: Collection homepage
                    twitterUsername:
                      type: string
                      description: The twitter username of the collection
                    discordUrl:
                      type: string
                      description: The discord URL of the collection
                    contract:
                      type: object
                      description: Contract-level data for a collection, such as contract type, name, and symbol.
                      properties:
                        address:
                          description: Address of the contract
                          type: string
                        name:
                          type: string
                          description: String - NFT contract name.
                        symbol:
                          type: string
                          description: String - NFT contract symbol abbreviation.
                        tokenType:
                          type: string
                          enum:
                            - ERC721
                            - ERC1155
                            - NO_SUPPORTED_NFT_STANDARD
                            - NOT_A_CONTRACT
                          description: String - For valid NFTs, 'ERC721' or 'ERC1155.' For invalid NFTs, a descriptive reason such as 'NO_SUPPORTED_NFT_STANDARD' if the input contract address doesn't support a known NFT standard, or 'NOT_A_CONTRACT' if there is no contract deployed at the input address.
                        contractDeployer:
                          type: string
                          description: String - Address that deployed the smart contract
                        deployedBlockNumber:
                          type: number
                          description: Number - The Block Number when the deployment transaction is successfully mined
                    totalBalance:
                      type: number
                      description: Sum of NFT balances across all token IDs held by the owner. For non-fungible tokens this will be equal to the `numDistinctTokensOwned`, but it may be higher if the user holds some fungible ERC1155 tokens.
                    numDistinctTokensOwned:
                      type: number
                      description: Number of distinct token IDs held by the owner. For non-fungible tokens this will be equal to the `totalBalance`, but it may be lower if the user holds some fungible ERC1155 tokens.
                    isSpam:
                      type: string
                      description: '"true" if contract is spam, else "false". **Only available on paid tiers.**'
                    displayNft:
                      type: object
                      description: Details of the display NFT for this contract. This NFT and its image can be used to represent the contract when displaying info about it.
                      properties:
                        tokenId:
                          description: One of the tokens from this contract held by the owner.
                          type: string
                        name:
                          description: 'The title of the token held by the owner i.e. "Something #22".'
                          type: string
                    image:
                      type: object
                      description: Details of the image corresponding to this contract
                      properties:
                        cachedUrl:
                          type: string
                          description: The Url of the image stored in Alchemy cache
                        thumbnailUrl:
                          type: string
                          description: The Url that has the thumbnail version of the NFT
                        pngUrl:
                          type: string
                          description: The Url that has the NFT image in png
                        contentType:
                          type: string
                          description: The Url of the image stored in Alchemy cache
                        size:
                          type: integer
                          description: The size of the media asset in bytes.
                        originalUrl:
                          type: string
                          description: The original Url of the image coming straight from the smart contract
              pageKey:
                type: string
              totalCount:
                type: string
                description: String - Total number of NFT collections held by the given address.
          examples:
            withoutMetadata:
              summary: Response (withMetadata = false)
              value:
                collections:
                  - address: '0x3a5051566b2241285BE871f650C445A88A970edd'
                    name: 'The Humanoids '
                    slug: thehumanoids
                    totalBalance: '1'
                    numDistinctTokensOwned: '1'
                    isSpam: false
                totalCount: 2120
                pageKey: 20ef9df5-0d81-42e5-b741-140f595a407b
            withMetadata:
              summary: Response (withMetadata = true)
              value:
                collections:
                  - name: The Humanoids
                    slug: thehumanoids
                    floorPrice:
                      marketplace: seaport
                      floorPrice: 0.37
                      priceCurrency: ETH
                    description: |-
                      The Humanoids (Gen 1) is a collection of 10,000 unique 3D 4K personalities.

                      [GEN 1.1 (CUSTOMIZABLE PFP)](https://opensea.io/collection/the-humanoids-gen-1-1/) | [DISCORD](https://discord.gg/thehumanoids) | [TWITTER](https://twitter.com/thehumanoids)

                      Stake your Gen 1 Humanoid and earn $ION to customize Gen 1.1 Humanoids using our proprietary Trait Factory.

                      Note: Holder Count is inaccurate as Humanoids are currently being staked.
                    externalUrl: http://thehumanoids.com
                    twitterUsername: thehumanoids
                    discordUrl: https://discord.gg/thehumanoids
                    contract:
                      address: '0x3a5051566b2241285BE871f650C445A88A970edd'
                      name: 'The Humanoids '
                      symbol: HMNDS
                      tokenType: ERC721
                      contractDeployer: '0xB8256c1c6654cedb9607644b07deC91Ca15fb9f6'
                      deployedBlockNumber: 13313830
                    totalBalance: '1'
                    numDistinctTokensOwned: '1'
                    isSpam: false
                    displayNft:
                      tokenId: '5880'
                      name: 'Humanoid #5880'
                    image:
                      cachedUrl: https://nft-cdn.alchemy.com/eth-mainnet/57dab2f078ca70e310c387064f66daaa
                      thumbnailUrl: https://res.cloudinary.com/alchemyapi/image/upload/thumbnailv2/eth-mainnet/57dab2f078ca70e310c387064f66daaa
                      pngUrl: https://res.cloudinary.com/alchemyapi/image/upload/convert-png/eth-mainnet/57dab2f078ca70e310c387064f66daaa
                      contentType: image/jpeg
                      size: 1898134
                      originalUrl: https://ipfs.io/ipfs/QmcjYgWMokcqnaSGZ31GVbGDe9V9z1KeNerRGfgeBEkn4k/5880.jpg
                totalCount: 2120
                pageKey: 03949322-9b2c-4fdd-aab6-1369e29fa5b2
  operationId: getCollectionsForOwner-v3
```
