## Introduction
* A token is a currency or digital asset in a limited environment on top of the blockchain
* 'Fungible' means that each token is equivalent to any other token
  * Just like a fiat currency (like USD or INR) or cryptocurrency (Ether)
* Doesn't replace the original cryptocurrency like Ether
* Similar to tokens used in gaming arcades or share certificates in a company
* Why create a token when Ether already exists?
  * Useful for defining and exchanging value within a closed system (not outside the ecosystem)
  * Inherently holds no value (unlike USD or Bitcoin) so can be mapped to various use cases
  * Can be controlled by the creating entity for various purposes
* Based on smart contracts - Code necessary to control supply, transfer, store balance, etc.
* Underlying transactions still happening on the blockchain
* Continue to require Ether (or equivalent) for gas fees

## Use cases
* Types
  * Utility - Currency to be spent in a specific environment like an online game or a company’s products 
  * Security - Define ownership of another asset, or of shares in a company or product
* Crowdfunding (Initial Coin Offering - ICO)
  * Equivalent to an IPO or a crowdfunding platform like Kickstarter, based on the stage and type 
  * Tokens bought in exchange for the underlying cryptocurrency (Ether) or a government currency (USD)
  * Can be of either type
    * Utility - Token can be used to buy company’s services or products, once available, usually at a discount 
    * Security - Profit sharing, ownership (STO, security token offering - shares), voting rights (DAO, decentralized autonomous organization)
* Online platform currency 
  * Entertainment - Internal currency of digital multiplayer games, digital casino, points on social sites 
  * Utility - Value store and currency in dapps - Royalty based music service, storage/backup service, freelance platform economy 
* Equivalent to physical assets
  * Represent ownership of physical assets like gold or commodities - similar to offline commodity funds

## Considerations
* Tokens implemented using a set of smart contracts
* Pre-defined logic to control overall supply, internal transactions, exchange with other currencies or services
* Anyone can write and deploy their token system
* Writing smart contracts is hard - especially for such a big use case
* Multiple standard specifications proposed and implemented to facilitate token management
* On Ethereum -> ERC-20 (Most popular), ERC-223, ERC-777
* Standards also allow popular wallets to implement and allow management of multiple tokens for normal users

## Fungible Tokens - ERC-20
* Initiated by Vitalik Buterin, detailed proposal by Fabian Vogelsteller in 2015
* Supports
  * Direct transfer - Normal transactions, buying, selling
  * Pre-approval to allow later expense by a third party - Escrow, subscriptions, auto-deduction based on task completion or delivery
* Contract API - Main functions:
  * _totalSupply_ - Returns the total tokens in the whole system
  * _balanceOf(address_owner)_ - Returns the balance of any account, given the _owner address 
  * _transfer(address_to, uint256_value)_ - Transfers _value to _to, from the current account 
  * _approve(address_spender, uint256_value)_ - Allows _spender to transfer upto total _value from the current account to anyone else, in multiple transactions
  * _transferFrom(address_from, address_to, uint256_value)_ - Allows a pre-approved sender to send _value from _from to _to
  * _name_ - Returns name of the token
  * _symbol_ - Returns symbol of the token

### Scale (on Ethereum)
* 400,000+ different tokens
* ~600 tokens with USD 1 million+ market cap 
* Combined market cap of just the top 10 tokens is USD 100+ billion
* ~300,000 daily active ERC-20 addresses
* Just Tether (number one token by market cap) has 10% of all Ethereum transactions

## Examples
* Tether - Pegged at $1, USD ~30 billion market cap, digital equivalent of cash currency
* Binance BNB - ~$300, USD ~5 billion market cap, utility token for various goods/services
* USD Coin - Pegged at $1, USD ~25 billion market cap, "Stable-coin" used in various financial services like payments, lending, trading, investing, etc.
* HEX - ~$0.01, USD ~50 billion market cap, Certificates of Deposit (CD) - interest generation based on investments like a fund
* Uniswap - ~$20, USD ~20 billion market cap, a cryptocurrency and token exchange with this token as the internal currency
* Chainlink - ~$20, USD ~20 billion market cap, the largest provider of oracles, internal currency for usage payment
* Theta - ~$6, USD ~6.5 billion market cap, video delivery using participating nodes who get paid in the token

## ICO Examples
* NEO - Smart contract-driven blockchain platform based in China
* Ethereum - Ethereum itself launched with an ICO
* NXT - Fintech focused blockchain platform
* Filecoin, Storj - Cloud storage with users participating as nodes for storage
* Bancor - Cryptocurrency trading exchange
* Sirin Labs - Security and privacy focused smartphones
* Brave BAT - Web browser with reward for advertising views