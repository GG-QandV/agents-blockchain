# Contracts By Owner

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-contracts-for-owner-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-contracts-for-owner-v-3.md)

# Contracts By Owner

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getContractsForOwner

getContractsForOwner - Retrieves all NFT contracts held by a specified owner address.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-contracts-for-owner-v-3

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
| orderBy | enum | No | Enum - ordering scheme to use for ordering NFTs in the response. If unspecified, NFTs will be ordered by contract address and token ID.   - transferTime: NFTs will be ordered by the time they were transferred into the wallet, with newest NFTs first. Note: This ordering is supported on Ethereum Mainnet, Optimism Mainnet, Polygon Mainnet, Base Mainnet, Arbitrum One, Polygon Amoy, Base Sepolia, Arbitrum Sepolia, Ethereum Sepolia, and Optimism Sepolia. |
| spamConfidenceLevel | enum | No | Enum - the confidence level at which to filter spam at. Confidence Levels:   - VERY_HIGH   - HIGH   - MEDIUM   - LOW The confidence level set means that any spam that is at that confidence level or higher will be filtered out. For example, if the confidence level is HIGH, contracts that we have HIGH or VERY_HIGH confidence in being spam will be filtered out from the response.  Defaults to VERY_HIGH for Ethereum Mainnet and MEDIUM for Matic Mainnet. **Please note that this filter is only available on paid tiers. Upgrade your account [here](https://dashboard.alchemy.com/settings/billing/).** |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getContractsForOwner
method: GET
operation:
  summary: Contracts By Owner
  description: getContractsForOwner - Retrieves all NFT contracts held by a specified owner address.
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
    - name: orderBy
      description: |-
        Enum - ordering scheme to use for ordering NFTs in the response. If unspecified, NFTs will be ordered by contract address and token ID.
          - transferTime: NFTs will be ordered by the time they were transferred into the wallet, with newest NFTs first. Note: This ordering is supported on Ethereum Mainnet, Optimism Mainnet, Polygon Mainnet, Base Mainnet, Arbitrum One, Polygon Amoy, Base Sepolia, Arbitrum Sepolia, Ethereum Sepolia, and Optimism Sepolia.
      in: query
      schema:
        type: string
        enum:
          - transferTime
      required: false
    - name: spamConfidenceLevel
      description: |-
        Enum - the confidence level at which to filter spam at.

        Confidence Levels:
          - VERY_HIGH
          - HIGH
          - MEDIUM
          - LOW

        > 📄 **This content also appears in [getContractsForOwner](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contracts-for-owner.md)** — see there for full details.

        **Please note that this filter is only available on paid tiers. Upgrade your account [here](https://dashboard.alchemy.com/settings/billing/).**
      schema:
        type: string
        enum:
          - VERY_HIGH
          - HIGH
          - MEDIUM
          - LOW
      in: query
      required: false
  responses:
    '200':
      description: Returns a list of NFT contracts held by the specified owner address.
      content:
        application/json:
          schema:
            type: object
            properties:
              contracts:
                type: array
                items:
                  type: object
                  description: The object that represents a smart contract and has all data corresponding to that contract
                  properties:
                    address:
                      type: string
                      default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
                    name:
                      description: The name of the contract, i.e. "Bored Ape Yacht Club".
                      type: string
                    symbol:
                      description: The symbol of the contract, i.e. BAYC.
                      type: string
                    totalSupply:
                      type: string
                      description: String - Total number of NFTs in a given NFT collection.
                    tokenType:
                      type: string
                    contractDeployer:
                      type: string
                      description: String - Address that deployed the smart contract
                    deployedBlockNumber:
                      type: number
                      description: Number - The Block Number when the deployment transaction is successfully mined
                    openseaMetadata:
                      type: object
                      description: Note that the OpenSea metadata object is currently only available on ETH and Polygon Mainnet. Please reach out to us at support@alchemy.com if you would like to access this data on other networks.
                      properties:
                        floorPrice:
                          type: number
                          description: NFT floor price
                        collectionName:
                          type: string
                          description: OpenSea collection name
                        safelistRequestStatus:
                          type: string
                          description: Collection approval status within OpenSea. For more info, see the Opensea docs at docs.opensea.io/reference/collection-model
                        imageUrl:
                          type: string
                          description: OpenSea CDN image URL
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
                        bannerImageUrl:
                          type: string
                          description: The banner image URL of the collection
                        lastIngestedAt:
                          type: string
                          description: The timestamp when the collection was last ingested by us
                    totalBalance:
                      type: number
                      description: Sum of NFT balances across all token IDs held by the owner. For non-fungible tokens this will be equal to the `numDistinctTokensOwned`, but it may be higher if the user holds some fungible ERC1155 tokens.
                    numDistinctTokensOwned:
                      type: number
                      description: Number of distinct token IDs held by the owner. For non-fungible tokens this will be equal to the `totalBalance`, but it may be lower if the user holds some fungible ERC1155 tokens.
                    isSpam:
                      type: boolean
                      description: '`True` if the contract is detected as spam contract. `False` if it is not spam or has not been evaluated by our system yet'
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
                description: String - Total number of NFT contracts held by the given address returned in this page.
          examples:
            withoutMetadata:
              summary: Response (withMetadata = false)
              value:
                contracts:
                  - address: '0x000386e3f7559d9b6a2f5c46b4ad1a9587d59dc3'
                    totalBalance: 912
                    numDistinctTokensOwned: 80
                    isSpam: true
                    tokenId: '0x0000000000000000000000000000000000000000000000000000000000000001'
                  - address: '0x0015f391949f25c3211063104ad4afc99210f85c'
                    totalBalance: 17
                    numDistinctTokensOwned: 6
                    isSpam: true
                    tokenId: '0x0000000000000000000000000000000000000000000000000000000000000002'
                  - address: '0x005b92d71a934dbe48e985b6469881cf4b0308fc'
                    totalBalance: 1
                    numDistinctTokensOwned: 1
                    isSpam: true
                    tokenId: '0x0000000000000000000000000000000000000000000000000000000000000003'
                totalCount: 2120
                pageKey: 20ef9df5-0d81-42e5-b741-140f595a407b
            withMetadata:
              summary: Response (withMetadata = true)
              value:
                contracts:
                  - address: '0x1C310c2fbB0D9755A6b918F990bC8D3504f2c684'
                    name: The Wonderful Husl Founder Cards
                    symbol: The Wonderful Husl Founder Cards
                    totalSupply: null
                    tokenType: ERC1155
                    contractDeployer: '0x0bdD0AEC835F92a465290cdd57b27FBd00376F53'
                    deployedBlockNumber: 15664554
                    openSeaMetadata:
                      floorPrice: null
                      collectionName: The Wonderful Husl Founder Cards
                      safelistRequestStatus: not_requested
                      imageUrl: https://i.seadn.io/gcs/files/754e38769c80c9d6188444dddb10ec80.png?w=500&auto=format
                      description: '[Husl](https://www.huslnft.xyz) is building the bridge between business and NFTs. Husl Founders are the driven, the passionate and the focused members of the community ready to change their future. Owning a Founders Card gets you exclusive perks, early access to business management, and discounts on managed services for your business as NFT. [Learn More](https://www.huslnft.xyz)'
                      externalUrl: https://www.huslnft.xyz
                      twitterUsername: null
                      discordUrl: null
                      lastIngestedAt: '2023-03-20T01:36:19.000Z'
                    totalBalance: '1'
                    numDistinctTokensOwned: '1'
                    isSpam: true
                    displayNft:
                      tokenId: '233'
                      name: null
                    image:
                      cachedUrl: https://nft-cdn.alchemy.com/eth-mainnet/d08d0d0fac8edf36ea09eae34b332814
                      thumbnailUrl: https://res.cloudinary.com/alchemyapi/image/upload/thumbnailv2/eth-mainnet/d08d0d0fac8edf36ea09eae34b332814
                      pngUrl: https://res.cloudinary.com/alchemyapi/image/upload/convert-png/eth-mainnet/d08d0d0fac8edf36ea09eae34b332814
                      contentType: video/mp4
                      size: 36190302
                      originalUrl: https://ipfs.io/ipfs/QmX2mM8r33W7KUBQSWXFAKNC2t654EXmWiX9vkrfrEaEnS
                totalCount: 2120
                pageKey: 03949322-9b2c-4fdd-aab6-1369e29fa5b2
  operationId: getContractsForOwner-v3
```
