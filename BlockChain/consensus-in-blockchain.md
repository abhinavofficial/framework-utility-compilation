# Consensus

## Consensus
We have learnt about [consensus](../parallel-concurrent-distributed-programming/distributed-programming/consensus-protocols.md) in distributed system. Consensus protocol is a fault-tolerant mechanism utilized in a blockchain to achieve an agreement on a single state of the network among distributed nodes.

Distributed ledger use independent computers called nodes to record, share and synchronize transactions in their specific electronic ledgers and do not keep data in a centralized system as in a traditional ledger. Each node therefore has to agree on which set of transactions (blocks) they can add to the distributed ledger, and hence consensus is required.

Consensus is central and helping grow the blockchain. Some nodes can fail, some can malfunction, - consensus helps them get consistent. We should also understand which consensus protocol can tolerate what kind of failure and throughput that it can deliver.

We have learnt about Paxos and raft in distributed computing. Let's see some more which is more specific to blockchain system, where tolerance for mistrust is more effectively managed.

## Proof of Work (PoW)

The PoW is a consensus algorithm that involves solving a computationally challenging puzzle in order to create new blocks in the Bitcoin blockchain.

Miners try to solve a crypto-puzzle which is the blockhash. This blockhash should satisfy some properties. This requires repeated trials. Once it is able to meet the required properties - it communicated with other of its success and broadcast your block to others.

So, the protocol is - first node to solve the crypto-puzzle will be allowed to add the block.

### Advantage
* No use of creating cybils (spoof its identity).
* It is also resilient to Byzantine failure. The node behaving in a byzantine way also needs to solve the crypto-puzzle.

### Cons
It is very power intensive.


## Proof of Elapsed Time (PoET)
Introduced by Intel in 2015. PoET is a consensus algorithm which allows permissioned blockchain network to determine who creates the next block. It works similar to lottery system that spreads the chances of winning equally across network participants, giving every node the same chance.

Intel came up with SGX, Intel Software Guard Extensions, a new trusted computing technology provided as a new solution to overcome the challenges in blockchain. It allows creation of trusted enclave in the Intel hardware. This is protected from other access. You can start a program in within this enclave, and generate a signature of that program, which can be used as an attestation of the program in the trusted enclave. All the contents are encrypted by the hardware keys, created and maintained by the hardware. This is not available to anyone including the OS unless they are part of the trusted chain.

This is now be used in the "permissioned" blockchain network. The central server, running a trusted enclave can load an initial program in participating node server, also running trusted enclave. The central server can now get attestation that the given program is running within all the trusted enclaves. It can give a random timer object (RTO) to each of the participating nodes - it allows program within each trusted enclave to sleep for random amount of time (this is now like a lottery system, which each server equal chance to win). The first one to wake up among the server can become the leader.

In this consensus protocol, there is no waste compute time and is hence more energy efficient. It can also scale more than PoW algorithm.

## Proof of Stake (PoS)
Proof of stake is a type of consensus mechanism used by blockchains for processing transactions and creating new blocks in a blockchain. It validates entries into a distributed database and keeps the database secure.

The node needs to be a part of committee to propose the new blocks in the blockchain.

Algorand, consensus protocol is where a set of nodes are elected amongst the participants nodes who can add the transaction blocks. Algorand (ALGO) is a blockchain platform and cryptocurrency designed to process similar to major payments processor like visa, master card. The Algorand blockchain uses PoS consensus mechanism and helps to secure the platform and reward the platform's operators. Verifiable Random Function, VRF, is a public-key pseudorandom function that takes a series of inputs, computes them, and produces a pseudorandom output. It also provides with a proof of authenticity that can be verified by anyone. So, a node takes its private key node and a random seed and generate a output - value and proof. The other nodes can use the public key of node and verify whether VRF was called.


## PBFT based Consensus Protocol
Practical Byzantine Fault Tolerance, pBFT send enough number of messages across the servers to gain consensus on the good node vs the bad one. It is difficult to implement PBFT in larger blockchain network because of the number of messages being exchanged. It is however, possible to implement in small permissioned blockchain network.

In a 2-phase commit, we have noted that the resilience against fail-stop failure of node. Paxos can handle this situation very effectively, so does Raft.

pBFT model mainly focuses on providing a practical Byzantine state machine duplication that tolerates Byzantine faults through an assumption that there are manipulated messages propagated by specific, independent node failures. The algorithm works in asynchronous systems and is optimized to be high-performance with an impressive overhead runtime and only a slight increase in latency.

Client sends the messages to primary. The primary node, also called as the leader node receives a request from the client and broadcasts the request to all the secondary nodes aka backup nodes in a pre-prepare message phase. The secondary nodes would send prepare-message to all the other nodes with its private key. If the primary get enough prepare messages (F+1, where F is the number of secondary nodes), it would move to commit-phase. The primary and secondary nodes perform the service requested and then send back a reply to the client. The client also needs to confirm that there are F+1 commit messages. This ensures that the primary is not behaving improperly. If it is detected that primary is not behaving correct, view change protocol would be triggered which is little more involved in this case since understanding the correct state is slightly more challenging and would require many more messages to determine the node with most accurate state.

Hotstuff, Facebook's modified pBFT called LibraBFT ([read](https://medium.com/ontologynetwork/hotstuff-the-consensus-protocol-behind-facebooks-librabft-a5503680b151)) is an optimized implementation to reduce the number of message exchanges.

Advantages of pBFT include energy efficiency, transaction finality and low reward variance.

## Why Blockchain becomes temper proof?
Every block has the hash of the previous block, except for the genesis block. The block hash is generated when the nodes have agreed posted PoW consensus protocol, which is then used in the next block. This is how blockchain progress.

If someone wants to temper one of the block, then it is recomputed the current block and all the subsequent blocks - which is very, very expensive.

## Longest chain rule, block size and mining fees
If there are more than one block which got successfully mined at the same time, the blockchain gets forked. A fork in blockchain technology is a radical change to a network protocol that makes earlier transactions and invalid block valid, or vice-versa. This can cause double spending since there is no cross validation happening. Forks in blockchain are classified into:
* Accidental fork and intentional forks
* Hard forks and soft forks

When nodes starts figuring out that there is a fork, they would start looking into which fork has the longest chain. Longest chain rule is when a new block arrives, and it extends the previous active chain, it just appends to the active chain i.e. the block of a valid version of the blockchain.

So, even your block can get mined and get added into the blockchain in few minutes, the actual confirmation of your block will take several hours in the blockchain to ensure that your block is not added in invalid fork.

If the block size is large, the propagation of the block is slow and chances of forking increases. It is hence, advised to keep the block size small, around 1MB. The size of the block is equal to the amount of data it stores. The maximum amount of data in a blockchain can store is referred to as the block size limit. Bitcoin and Litecoin puts this limit as 1MB which Bitcoin cash limits to 32MB.

If the block size is too small, the throughput is reduced. 

[Mining](mining.md) fees pay for the computing ability for a transaction to be authenticated on a cryptocurrency network. Mining fees are given to the computer, or miner that does the work of verifying the next block of transactions added to the blockchain. So, higher the fees, higher is chances of the transaction being mined.

## Block withholding attack
It mines but withholds with itself and keeps building within itself without publishing. When the time is appropriate, it publishes and attaches this to the blockchain. It may succeed as well due to the longest chain rule. It is also mean that the miner directly discards the block even though the block is legal. This attack will make the mining pool loose all the bitcoin rewards that are there within the block.