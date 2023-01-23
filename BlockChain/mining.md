# Mining

## Cryptographic Challenge
In large blockchain network, like Bitcoin and Ethereum, solving the crypto puzzle has become so complex that special Application specific integrated circuits (ASIC) has to designed just to generate hashes. We now understand that ultimate goal is to generate the bash for the block which meets minimum criteria.

* Block hash includes all the elements on a block, including the nonce
* Hashing (ex. SHA-256) is a one-way function with high variability. Bitcoin network specifies that to calculate the hash of the block, SHA-256 algorithm is run twice (means block hash is hash of a hash)
* Changing only the nonce, a miner has to create the hash below a given target value
* Target: Find a hash with value below the target value, practically equivalent to finding a hash
with n number of leading zeros
* Difficulty: Computational cost of finding a target
* Lower the target, higher the difficulty
* Every n (2016 [there are 2016 10-mins in 2 week = 6*24*14] for bitcoin) blocks, the target is lowered, and hence difficulty increased
* Change is based on keeping a predictable block creation rate. If block creation is become faster, this means that mining has become easier or there are increased number of miners in the network and hence increased difficulty yields predictable results.
* Example: 10 minutes average time for bitcoin and 10-19 sec for Ethereum
  * Tradeoff between enough complexity and avoiding wastage due to chain splits

> A chain-split is referred to as a permanent branching of the Bitcoin blockchain. This happens when blocks of transactions are created by nodes operating a new version of Bitcoin which has less restrictive consensus rules.

## Process
* Mining Node: Full validation node which attempts to solve the crypto puzzle and create blocks. Full node by default, save the complete blockchain data. Full node includes information about all blocks and the complete records of transactions carried out within these blocks. Miners would typically be full node.
* People combine their mining nodes in a common pool to have a better chance of reward. For example, Antpool is the world's leading cryptocurrency mining platform based in China and is owned by BitMain. Antpool mines about 15% of all blocks. It is committed to providing users with high-quality multi-currency mining services.
* Transaction selection:
  * Transactions can add fees/rewards for miners
  * Miners can decide to prioritize transactions based on highest fees or based on a cut-off reward
  * They may optionally add lower fee or "free" transactions if there is space
* Number of transactions depend on block size; 500+ average for bitcoin
* Miner mines and finds an appropriate hash
* They add the block and propagate the information to connected nodes
* Other full nodes validate the new block and all transactions
* Confirmation (6 blocks for Bitcoin and 12 blocks for Ethereum) from most of the network leads to legitimacy

> Partial node aka lightweight node does not maintain the whole copy of the blockchain ledger but only has the hash value of the transaction and whole transaction is accessed using only hash value. Partial node have low computation power and low storage.

## Issues
* Block propagation delay after mining due to geographical and network constraints
  * Increases the chances of attacks or forks
* Orphan/Uncle/Ommer blocks: Blocks mined at roughly the same time but not added to the chain (and hence not accepted), probable if difficulty is low and propagation delays are high.
  * Treated differently by different blockchains in terms of rewards. In Bitcoin, referred to as Orphan/Ommer blocks, the miners do not get any rewards. In Ethereum, referred to as uncle blocks, the miners are still rewarded.
* Block withholding attack, also called Finney attack named after the discoverer of the issue, Hal Finney.
  * Mine a block but don’t publish it
  * Initiate transaction with someone for a digital good and receive it
  * Immediately publish the block with another conflicting transaction
  * Digital good transaction will be invalid now due to lack of credit and will be invalidated
  * Miner keeps the digital good without spending, or "double spending"
  * Consensus in bitcoin is to wait for 6 confirmations (new blocks) before transaction acceptance
  * Creating and withholding six blocks before someone else can create even one is exponentially harder
* 51% attack - Node collusion. This is an attack by a group of miners who control more than 50% of the network's mining hash rate. If the miners own 51% of the nodes on the network it gives the controlling parties the power to modify the blockchain.
  * Gain control of at least 51% of the hashing (mining) power in a blockchain
  * Attackers will control the blocks created and the transactions confirmed
  * Can double-spend, revert older transactions, prevent new transactions from benign node
  * Leads to monetary loss and the reliability/security of the whole blockchain
  * Famous attacks on Ethereum Classic (ETC), Bitcoin Gold (BTG), etc. ETC faced this thrice in a single month in 2015.
> Ethereum Classic (ETC) is a blockchain-based, decentralised, open-source computing platform permanently forked from Ethereum. It allows developers to build and deploy smart contracts which are autonomous, self-executing code blocks that trigger actions based on predefined conditions. There was a vulnerability in DAO (Decentralised Autonomous Organization which is a software running on a blockchain that gives users a built-in model for the collective management of its code) which led hackers to siphon off 1/3 of the ethereum ICO it generated (50-60 million dollars).
> Bitcoin Gold (BTG) is a cryptocurrency with fundamentals of Bitcoin. It can be mined on common GPUs rather than speciality ASIC. ASICs tend to utilize mining to some big players, anyone can mine again which helps in restoring decentralisation and independence. Has 100 odds nodes, mostly in Germany.

Ethereum 2.0 use PoS. In these, 51% even if happens, will happen only once. 