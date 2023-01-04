# BlockChain

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

### Key issues

#### Centralization

* Power tends to corrupt and absolutely power corrupts absolutely
* Our current systems are completely based on centralization from our real-life to digital processes.
* For example, in a football match, we trust the referees to make the correct decisions.
* In SSL implementation we trust the central authorities to provide security and correct certificates.
* What if the central authorities are not trustworthy. For example, DigiNotar a trusted CA fraudulently issued the certificates, because of which thousands of Gmail users were compromised.
* Similarly, we had multiple public sector banks where centralization causes a major issue.
* Centralization also causes a single point of failure and ease of attacks for hackers.

Trust should be a component in a distributed systems.

A single point of failure (SPOF) is a part of a system that, if it fails, will stop the working of the entire system. It is undesirable for systems that demand high availability and reliability and are at potential risk. Hence, peer 2 peer networks have a distributed architecture.

#### Security

* Security has become a major concern for digital system. Especially in this shifting paradigm for office cultures after COVID
* Security could be related to fraudulent transactions, malware or internal breaches.
* For example, Zoom was hacked where the hackers were able to take user passwords.
* Another example was with MiraBotnet which caused a huge DDOS attack on multiple services and outage of applications.

> DDOS stands for distributed denial of service attack. It is a malicious attempt to make an online system unavailable by disrupting it with traffic from multiple sources.

#### Integrity and Transparency

* Apart from security, integrity and transparency also make up a major aspect of the digital cycle.
* Integrity defines where the data is correct or not. Suppose you send across an email that gets modified by a man in the middle, your important information would never reach the other user.
* Similarly, transparency is also required in solutions to build up trust. There could be hidden dealings if transparency is not maintained for solutions.
* Integrity and transparency had been major concern for digital systems. On top of centralization adds in much more difficulty to get transparency in systems.

#### Double Spending

* Double spending is a persistent problem with digital assets.
* Anything can be copied - be it pirated movies, games, coins, software, etc.
* A user can send the same thing to two users and there is no way for the users to recognize that they are getting the copy.
* In our tradition systems, we use central authorities. For example, we have banks to verify the transactions bit what if the banks or central authorities are corrupted?

Double spending is a flaw in digital cash that means that the same units of a cryptocurrency are spent twice in order to deceive the recipient of those funds.


### Solution with Blockchain

#### Decentralized control

* Blockchains create a P2P network with in turn allows multiple parties which initially do not trust each other to share information without requiring a central control.
* Same data stored at multiple participants provide us security from attacks like DDOS, without the need of backup machines.
* Cost savings are also provided as it saves billions of dollars which are spent on safeguarding central repositories from hackers.
* Blockchain provides a same shared system of records simultaneously for everyone who is connected to the network.
* The trust is established by cryptographic protocols running behind the blockchain technology
* All the parties must agree to make a chain in blockchain which is nearly impossible.

> Peer 2 peer network is a simple network of computers where a group of computers are linked together with equal responsibilities and permissions for processing data. It is a decentralized network communications model consisting of a group of devices (aka nodes) that collectively save and share files where each node behaves as an individual peer. P2P communication doesn't have a centralized administration or server, hence all nodes have equal power and perform the same tasks.

#### Enhanced Security with Cryptocurrency

* Transactions which are submitted on the blockchain are encrypted and linked to previous transactions.
* Data/information is stored across a network of distributed computers instead of on a single server.
* Blockchain prevents fraud and unauthorized activity.
* Cryptography protocols are used for building up keys and hashes which provides a built-in security.
* Networking protocols like TCP and Gossip Protocol adds in additional layer of security.
* Cryptographic fingerprint (hash of the block) is unique for each block.

#### Built in Integrity and Transparency

* Blockchain technology in regard to public systems like Bitcoin and Etherum, distinguishes it from traditional database technology as it is publicly verifiable, which is enabled by integrity and transparency.
* Every user can be sure that the data they are retrieving is uncorrupted and unaltered since the moment it was recorded.
* Immutability adds into the integrity.
* Every user can verify data/transactions appended over the blockchain.
* Blockchain keeps on growing like a expanding archives database which tracks the history while also providing a real-time views.
* Merkle tree ensures the integrity of data by hashing the transactions to a single root.

A root hash, aka merkle root, is a special type of hash generated in order to facilitate the verification of data within a merkle tree.

#### Double Spending solution with P2P sharing of data

* Double spending solution is provided as blocks are appended and transactions are also in links.
* If a person tries to send the same digital currency to 2 different individuals then once one of the transaction is confirmed, it is already linked to parent transaction. The other transaction who is asking to link to the parent transaction will be rejected.
* Even somehow the user submits the same transaction and 2 different blocks are created forming 2 different chains. After some interval the long chain rule will make sure that only the chain which is most worked upon is taken into the main chain and the other transactions will not get confirmed.