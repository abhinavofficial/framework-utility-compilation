# Blockchain

## Core Concepts

## BlockChain

The private or permissioned blockchain have an authorization scheme to identify which participant/user is entering and has access to the blockchain network. Whereas, in a public or permissionless blockchain system, anyone can join the network and there are no restrictions when it comes to participation.

If we are running any particular protocol, what we see is blockchain that is the state of the transaction, this is available with all the nodes of the network.

There are three types of blocks that exist in different blockchains.
* Genesis block: It is the first block of a blockchain
* Valid blocks are all blocks that have been mined and added to the blockchain.
* Orphan blocks: Blocks that are not part of the blockchain network

This is how a distributed ledge is created and is available with all the nodes. There are pool of pending transactions which has to be confirmed. Any node in the network can pick up a transaction and convert it in a block and can attempt to add this block into a blockchain - this is the proposal state. To get the confirmation, rest of the nodes have to agree. For this agreement to happen, a consensus protocol has to be run.

Consensus protocols are the backbone of blockchain. It comprises some specific purposes which include collaboration, cooperation, equal rights to every node, coming to an agreement, and mandatory participation of each node in the consensus process. There are many consensus protocols. One of them is Proof of Work (PoW) - that involves solving a computationally challenging puzzle in order to create new blocks in the Bitcoin blockchain.

In a bitcoin blockchain, the genesis block has the hash generated for that block. The following block has the hash of the previous hash and additional property and some leading zeros in the hash key. Each node agrees to solve this crypto puzzle. Whichever node solves it first makes the announcement to the rest, so the proposal for that node is accepted by the rest. This is how a blockchain progresses.

Peer to peer network is based on zero trust, which no node trusts the other node. Each node keep its copy of blockchain and can verify if the next block getting added into the chain is the correct block. This means that building a blockchain is a very expensive process, and we can get only few blocks getting added per second as compared to traditional databases, where it can be in tens of thousands of records per sec. So, unless trust is the most important thing, we should not be implementing blockchain.

Key difference between blockchain and distributed database is that 
* In distributed database, part of the transaction is run on multiple node across different sites either on the same network or on entirely different networks and as part of commit protocol, either all or none is accepted as change. Looking closely, each of these nodes in that sense are trusted nodes.
* Immutable ledger in blockchain represents any record that has the ability to remain unaltered. It cannot be changed and therefore the data cannot be changed with ease, which in turn tightens the security.

## Issues in current systems

| Issues with having a single Central Controller.                                                                     | Resolution |
|---------------------------------------------------------------------------------------------------------------------|------------|
| Failure in one controller, system availability becomes an issue                                                     ||
| It is easy to attack a single controller, leading to security issues                                                ||
| Since the data is kept at one single place, if the data is corrupted, the integrity of the entire system is at risk ||
| Defects in a single controller is difficult to be checked.                                                          ||