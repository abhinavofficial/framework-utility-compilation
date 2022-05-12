# Consensus
Consensus: Coordinating processes agree on a data value that they require during computation. Examples:
* Choosing transactions to commit on a data store
* Selecting an order of those transactions

Consensus protocol: A distributed algorithm used to agree on data value(s)
* Non-trivial
* Depends on input to some process
* Voting: agreeing on a majority value
* Handles limited faulty processes

A correct process: Has not experienced a failure

## Consensus Protocol - Shared Memory Model

Let's assume there are n processes, P1, P2, ..., Pn sharing a common memory. All these processes can do a read or write on a shared memory but in exclusive fashion. Let's assume that the process will be conducting a full atomic operation - **compare-and-swap**. So, if all the processes start with the initial value of x which is present in the shared memory. The one process which is able to match this value and swap its value to y. Other process will fail the comparison and will not be able to do a swap and instead read y. At the end of the process, all the processes will have one single value of y being agreed (and a consensus is reached).

**compare-and-swap** algorithm can allow up to n-1 failures - this means, the algorithm will terminate successfully even if there are n-1 process failures. It is therefore called **universal primitive??**. This is also a wait free protocol - none of the other process need to wait because other processes are failing. If you contrast this with the highest ID leader algorithm - unless all the nodes have put in their IDs for evaluation, the process cannot terminate and all participating nodes will be blocked. This is an example of wait for protocol. We prefer wait free protocol over wait for protocol in distributed system.

**fetch-and-add** algorithm is another algorithm for consensus but is less powerful than compare-and-swap.

## Consensus Protocol - Message Passing Model

In general, in a loosely coupled system, we have message passing model which uses ```send()``` and ```receive()``` primitives.

One major performance difference between accessing a shared memory over a BUS and over a Network topology is drastically different - BUS is high speed and is comparable to access time of memory locations. 

It is possible to simulate shared memory over message passing distributed system. Let us consider, a multinode distributed system where a node can access other node's memory over the network - this can be very slow. Now, lets assume that we replicate memory to each node so that the reads can happen locally and thus the reads will be access memory over BUS.  For writes, we can serialize and apply them in the same order over each node. In a system, where reads are very high, this may work very well.

We can also partition memory and assign part of memory to individual nodes so as data requirement to compute becomes local to that node. Locality for reference (????) i.e. when the node need other data partition for its computation, the partition can be re-assigned.

Atomic Broadcast - Network can guarantee that it passes the received "broadcast" values from each connected node to other nodes in the same order. This can make consensus easy if we have a algorithm which states that the first value that nodes received would be an agreed value. One way to implement this is that every node has to get a serial number for its message before it sends it out or all the messages are serialized by one of the nodes or some other ways. Zookeeper uses atomic broadcast (popularly known as ZAP Zookeeper Atomic Protocol) to enforce on connected nodes a way of receiving values in the same order they are sent.

## Consensus Paxos

Read more [here](consensus-paxos.md)

## Consensus Raft

Read more [here](consensus-raft.md)