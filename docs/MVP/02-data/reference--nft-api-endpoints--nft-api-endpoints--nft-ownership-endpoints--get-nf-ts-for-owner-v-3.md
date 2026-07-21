# NFTs By Owner

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-nf-ts-for-owner-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-nf-ts-for-owner-v-3.md)

# NFTs By Owner

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getNFTsForOwner

getNFTsForOwner - Retrieves all NFTs currently owned by a specified address. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-nf-ts-for-owner-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| owner | string | Yes | String - Address for NFT owner (can be in ENS format for Eth Mainnet). |
| contractAddresses[] | string[] | No | Array of contract addresses to filter the responses with. Max limit 45 contracts. |
| withMetadata | boolean | No | Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`. |
| orderBy | enum | No | Enum - ordering scheme to use for ordering NFTs in the response. If unspecified, NFTs will be ordered by contract address and token ID.   - transferTime: NFTs will be ordered by the time they were transferred into the wallet, with newest NFTs first. Note: This ordering is supported on Ethereum Mainnet, Optimism Mainnet, Polygon Mainnet, Base Mainnet, Arbitrum One, Polygon Amoy, Base Sepolia, Arbitrum Sepolia, Ethereum Sepolia, and Optimism Sepolia. |
| excludeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. NFTs that match one or more of these filters will be excluded from the response. May not be used in conjunction with includeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |
| includeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. Only NFTs that match one or more of these filters will be included in the response. May not be used in conjunction with excludeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |
| spamConfidenceLevel | enum | No | Enum - the confidence level at which to filter spam at. Confidence Levels:   - VERY_HIGH   - HIGH   - MEDIUM   - LOW The confidence level set means that any spam that is at that confidence level or higher will be filtered out. For example, if the confidence level is HIGH, contracts that we have HIGH or VERY_HIGH confidence in being spam will be filtered out from the response.  Defaults to VERY_HIGH for Ethereum Mainnet and MEDIUM for Matic Mainnet. **Please note that this filter is only available on paid tiers. Upgrade your account [here](https://dashboard.alchemy.com/settings/billing/).** |
| tokenUriTimeoutInMs | integer | No | No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0. |
| pageKey | string | No | String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results. |
| pageSize | integer | No | Number of NFTs to be returned per page. Defaults to 100. Max is 100. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForOwner?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getNFTsForOwner
method: GET
operation:
  summary: NFTs By Owner
  description: getNFTsForOwner - Retrieves all NFTs currently owned by a specified address. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
  tags:
    - NFT Ownership Endpoints
  parameters:
    - name: owner
      description: String - Address for NFT owner (can be in ENS format for Eth Mainnet).
      schema:
        type: string
        default: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
      in: query
      required: true
    - name: contractAddresses[]
      description: Array of contract addresses to filter the responses with. Max limit 45 contracts.
      schema:
        type: array
        items:
          type: string
      in: query
    - name: withMetadata
      description: Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`.
      schema:
        type: boolean
        default: true
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
    - name: tokenUriTimeoutInMs
      description: No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0.
      in: query
      schema:
        type: integer
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
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
  responses:
    '200':
      description: Returns the list of all NFTs owned by the given address and satisfying the given input parameters.
      content:
        application/json:
          schema:
            type: object
            properties:
              ownedNfts:
                type: array
                description: Array of the NFT objects corresponding to the NFTs owned by the owner
                items:
                  type: object
                  description: The object that represents an NFT and has all data corresponding to that NFT
                  properties:
                    contract:
                      type: object
                      description: The contract object that has details of a contract
                      properties:
                        address:
                          description: Address of the held contract
                          type: string
                        name:
                          type: string
                          description: String - NFT contract name.
                        symbol:
                          type: string
                          description: String - NFT contract symbol abbreviation.
                        totalSupply:
                          type: string
                          description: String - Total number of NFTs in a given NFT collection.
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
                        isSpam:
                          type: string
                          description: '"true" if contract is spam, else "false". **Only available on paid tiers.**'
                        spamClassifications:
                          description: List of reasons why a contract was classified as spam. **Only available on paid tiers.**
                          type: array
                          items:
                            type: string
                    tokenId:
                      type: string
                      default: '44'
                    tokenType:
                      type: string
                    name:
                      type: string
                      description: String - Name of the NFT asset.
                    description:
                      type: string
                      description: String - Brief human-readable description
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
                    raw:
                      type: object
                      description: Raw details of the NFT like its tokenUri and metadata info obtained directly from the smart contract
                      properties:
                        tokenUri:
                          type: string
                          description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                        metadata:
                          type: object
                          description: Relevant metadata for NFT contract. This is useful for viewing image url, traits, etc. without having to follow the metadata url in tokenUri to parse manually.
                          properties:
                            image:
                              type: string
                              description: String - URL to the NFT asset image. Can be standard URLs pointing to images on conventional servers, IPFS, or Arweave. Most types of images (SVGs, PNGs, JPEGs, etc.) are supported by NFT marketplaces.
                            name:
                              type: string
                              description: String - Name of the NFT asset.
                            description:
                              type: string
                              description: String - Human-readable description of the NFT asset. (Markdown is supported/rendered on OpenSea and other NFT platforms)
                            attributes:
                              type: array
                              items:
                                type: object
                                properties:
                                  value:
                                    type: string
                                  trait_type:
                                    type: string
                              description: Object - Traits/attributes/characteristics for each NFT asset.
                        error:
                          type: string
                          description: String - A string describing a particular reason that we were unable to fetch complete metadata for the NFT.
                    collection:
                      type: object
                      description: The collection object that has details of a collection
                      properties:
                        name:
                          type: string
                          description: String - Collection name
                        slug:
                          type: string
                          description: String - OpenSea collection slug
                        externalUrl:
                          type: string
                          description: String - URL for the external site of the collection
                        bannerImageUrl:
                          type: string
                          description: String - Banner image URL for the collection
                    tokenUri:
                      type: string
                      description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                    timeLastUpdated:
                      type: string
                      description: String - ISO timestamp of the last cache refresh for the information returned in the metadata field.
                    acquiredAt:
                      type: object
                      description: Only present if the request specified `orderBy=transferTime`.
                      properties:
                        blockTimestamp:
                          type: string
                          description: Block timestamp of the block where the NFT was most recently acquired.
                        blockNumber:
                          type: string
                          description: Block number of the block where the NFT was most recently acquired.
                    animation:
                      type: object
                      properties:
                        cachedUrl:
                          type: string
                        contentType:
                          type: string
                        size:
                          type: integer
                        orginalUrl:
                          type: string
                    mint:
                      type: object
                      properties:
                        mintAddress:
                          type: string
                          description: Address that minted the NFT
                        blockNumber:
                          type: integer
                          description: Block number when the NFT was minted
                        timestamp:
                          type: string
                          description: Timestamp when the NFT was minted
                        transactionHash:
                          type: string
                          description: Transaction hash of the mint transaction
                    owners:
                      type: array
                      description: List of all addresses that own the given NFT.
                      items:
                        type: string
              totalCount:
                type: integer
                description: Integer - Total number of NFTs (distinct `tokenIds`) owned by the given address.
              pageKey:
                type: string
              validAt:
                type: object
                description: Block Information of the block as of which the corresponding data is valid
                properties:
                  blockNumber:
                    type: integer
                    description: The block number above information is valid as of
                  blockHash:
                    type: string
                    description: The block hash above information is valid as of
                  blockTimestamp:
                    type: string
                    description: The block timestamp above information is valid as of
          examples:
            byDefault:
              summary: Response (By Default)
              value:
                ownedNfts:
                  - contract:
                      address: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                      name: DuskBreakers
                      symbol: DUSK
                      totalSupply: '10000'
                      tokenType: ERC721
                      contractDeployer: '0x9c78DDec1F16685ee6E58637a640514A1eD87BC4'
                      deployedBlockNumber: 13736379
                      openSeaMetadata:
                        floorPrice: 0.0582
                        collectionName: DuskBreakers
                        safelistRequestStatus: verified
                        imageUrl: https://i.seadn.io/gae/LGbFRVdClz6-HDd-7WZKONJ5Ody0sBXTvFOQL71BYo3j2iU2wWCX_zlk-Zs0KEhq1qgXViF-6aG_0WS2MdIVNJx2GRSIIYTiciuf-A?w=500&auto=format
                        description: "Being a DuskBreaker means joining a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology. You will be part of a community that directly influences the development of upcoming interactive media and gaming experiences within the DuskBreakers universe. Each of you will play an important role in building out this world. You break it, you take it! \r\n\r\nVisit [DuskBreakers](https://duskbreakers.gg) to learn more."
                        externalUrl: http://duskbreakers.gg
                        twitterUsername: duskbreakers
                        discordUrl: https://discord.gg/duskbreakers
                        lastIngestedAt: '2023-04-19T17:25:59.000Z'
                      isSpam: null
                      spamClassifications: []
                    tokenId: '28'
                    tokenType: ERC721
                    name: 'DuskBreaker #28'
                    description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                    image:
                      cachedUrl: https://nft-cdn.alchemy.com/eth-mainnet/1f9e8be3feb42b5b66452537a4032668
                      thumbnailUrl: https://res.cloudinary.com/alchemyapi/image/upload/thumbnailv2/eth-mainnet/1f9e8be3feb42b5b66452537a4032668
                      pngUrl: https://res.cloudinary.com/alchemyapi/image/upload/convert-png/eth-mainnet/1f9e8be3feb42b5b66452537a4032668
                      contentType: image/png
                      size: 1474037
                      originalUrl: https://duskbreakers.gg/breaker_images/28.png
                    raw:
                      tokenUri: https://api.duskbreakers.gg/metadata/duskbreakers/28
                      metadata:
                        name: 'DuskBreaker #28'
                        description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                        image: https://duskbreakers.gg/breaker_images/28.png
                        external_url: https://duskbreakers.gg/
                        attributes:
                          - value: Locust Rider Armor (Red)
                            trait_type: Clothes
                          - value: Base Drone (Blue)
                            trait_type: Drone
                          - value: Thin
                            trait_type: Eyebrows
                          - value: Button
                            trait_type: Nose
                          - value: Mohawk (Black)
                            trait_type: Hair
                          - value: Almond 2 (Red)
                            trait_type: Eyes
                          - value: Big Smile (Purple)
                            trait_type: Mouth
                          - value: Light Brown
                            trait_type: Skin Tone
                          - value: Yellow
                            trait_type: Background
                          - value: Facepaint (Stripe)
                            trait_type: Face Augments
                      error: null
                    tokenUri: https://api.duskbreakers.gg/metadata/duskbreakers/28
                    timeLastUpdated: '2023-04-19T21:25:39.563Z'
                    balance: '1'
                  - contract:
                      address: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                      name: DuskBreakers
                      symbol: DUSK
                      totalSupply: '10000'
                      tokenType: ERC721
                      contractDeployer: '0x9c78DDec1F16685ee6E58637a640514A1eD87BC4'
                      deployedBlockNumber: 13736379
                      openSeaMetadata:
                        floorPrice: 0.0582
                        collectionName: DuskBreakers
                        safelistRequestStatus: verified
                        imageUrl: https://i.seadn.io/gae/LGbFRVdClz6-HDd-7WZKONJ5Ody0sBXTvFOQL71BYo3j2iU2wWCX_zlk-Zs0KEhq1qgXViF-6aG_0WS2MdIVNJx2GRSIIYTiciuf-A?w=500&auto=format
                        description: "Being a DuskBreaker means joining a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology. You will be part of a community that directly influences the development of upcoming interactive media and gaming experiences within the DuskBreakers universe. Each of you will play an important role in building out this world. You break it, you take it! \r\n\r\nVisit [DuskBreakers](https://duskbreakers.gg) to learn more."
                        externalUrl: http://duskbreakers.gg
                        twitterUsername: duskbreakers
                        discordUrl: https://discord.gg/duskbreakers
                        lastIngestedAt: '2023-04-19T17:25:59.000Z'
                      isSpam: null
                      spamClassifications: []
                    tokenId: '29'
                    tokenType: ERC721
                    name: 'DuskBreaker #29'
                    description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                    image:
                      cachedUrl: https://nft-cdn.alchemy.com/eth-mainnet/4eb0b7f434746250ff3c8200d10a2226
                      thumbnailUrl: https://res.cloudinary.com/alchemyapi/image/upload/thumbnailv2/eth-mainnet/4eb0b7f434746250ff3c8200d10a2226
                      pngUrl: https://res.cloudinary.com/alchemyapi/image/upload/convert-png/eth-mainnet/4eb0b7f434746250ff3c8200d10a2226
                      contentType: image/png
                      size: 1480183
                      originalUrl: https://duskbreakers.gg/breaker_images/29.png
                    raw:
                      tokenUri: https://api.duskbreakers.gg/metadata/duskbreakers/29
                      metadata:
                        name: 'DuskBreaker #29'
                        description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                        image: https://duskbreakers.gg/breaker_images/29.png
                        external_url: https://duskbreakers.gg/
                        attributes:
                          - value: Standard Issue Armor 1 (Orange)
                            trait_type: Clothes
                          - value: Dark Metal
                            trait_type: SmartSkin
                          - value: Base Drone (Purple)
                            trait_type: Drone
                          - value: Thin
                            trait_type: Eyebrows
                          - value: Broad
                            trait_type: Nose
                          - value: Slick Back (Red)
                            trait_type: Hair
                          - value: Sharp (Blue)
                            trait_type: Eyes
                          - value: Smirk (Neutral)
                            trait_type: Mouth
                          - value: Tan
                            trait_type: Skin Tone
                          - value: Purple
                            trait_type: Background
                      error: null
                    tokenUri: https://api.duskbreakers.gg/metadata/duskbreakers/29
                    timeLastUpdated: '2023-04-19T21:25:39.704Z'
                    balance: '1'
                  - contract:
                      address: '0x209cE666978779756Ae1E747608cD93e4dFf45fD'
                      name: Knight of Chains Genesis
                      symbol: Knight of Chains Genesis
                      totalSupply: null
                      tokenType: ERC1155
                      contractDeployer: '0xA92520aFF50c5A1a4d25FCF90c972AA49EbE5299'
                      deployedBlockNumber: 14847327
                      openSeaMetadata:
                        floorPrice: null
                        collectionName: Knight of Chains Genesis.
                        safelistRequestStatus: not_requested
                        imageUrl: https://i.seadn.io/gae/eRhkkVikIOW_-lDc1moMrZlTcd5DPygPRmTJ69Anb-CfG_RMAxIsichM5kDvfdnXc6gfnKuGZOFCbP_58pUvz57TyUeNbFMKGydHoac?w=500&auto=format
                        description: |-
                          [The KnightsOfChain] (https://knightsofchain.link) is an exclusive community that can only be entered by owning a Knight.

                          Visit [Website](https://knightsofchain.link) and get your benefits.

                          (Genesis Knights #1-#31 were pre-minted by the team, and are held by high ranking community members. OG Knights #32-#231 have special benefits.)
                        externalUrl: https://knightsofchain.link
                        twitterUsername: null
                        discordUrl: null
                        lastIngestedAt: '2023-03-20T03:52:07.000Z'
                      isSpam: null
                      spamClassifications: []
                    tokenId: '97'
                    tokenType: ERC1155
                    name: null
                    description: null
                    image:
                      cachedUrl: null
                      thumbnailUrl: null
                      pngUrl: null
                      contentType: null
                      size: null
                      originalUrl: null
                    raw:
                      tokenUri: https://knightsofchain.link/ipfs/97
                      metadata: {}
                      error: null
                    tokenUri: https://knightsofchain.link/ipfs/97
                    timeLastUpdated: '2023-04-20T15:44:29.965Z'
                    balance: '1'
                totalCount: 3
                validAt:
                  blockNumber: 17091500
                  blockHash: '0x2a34a65c4e0cd7fdf187d6a497214ad2bee255d2d3501868a6b8c09b4d1261bd'
                  blockTimestamp: '2023-04-21T01:25:59Z'
                pageKey: null
            withoutMetadata:
              summary: Response (withMetadata = false)
              value:
                ownedNfts:
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '28'
                    balance: '1'
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '29'
                    balance: '1'
                totalCount: 2
                validAt:
                  blockNumber: 17091500
                  blockHash: '0x2a34a65c4e0cd7fdf187d6a497214ad2bee255d2d3501868a6b8c09b4d1261bd'
                  blockTimestamp: '2023-04-21T01:25:59Z'
                pageKey: null
            withContractFiltering:
              summary: Response (with contract filtering)
              value:
                ownedNfts:
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '28'
                    balance: '1'
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '29'
                    balance: '1'
                totalCount: 2
                validAt:
                  blockNumber: 17091500
                  blockHash: '0x2a34a65c4e0cd7fdf187d6a497214ad2bee255d2d3501868a6b8c09b4d1261bd'
                  blockTimestamp: '2023-04-21T01:25:59Z'
                pageKey: null
            withPagination:
              summary: Response (with pagination)
              value:
                ownedNfts:
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '28'
                    balance: '1'
                  - contractAddress: '0x0bEed7099AF7514cCEDF642CfEA435731176Fb02'
                    tokenId: '29'
                    balance: '1'
                totalCount: 2
                validAt:
                  blockNumber: 17091500
                  blockHash: '0x2a34a65c4e0cd7fdf187d6a497214ad2bee255d2d3501868a6b8c09b4d1261bd'
                  blockTimestamp: '2023-04-21T01:25:59Z'
                pageKey: 88434286-7eaa-472d-8739-32a0497c2a18
  operationId: getNFTsForOwner-v3
```
