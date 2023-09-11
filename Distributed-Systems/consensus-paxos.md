# Paxos

Paxos is one of the protocol that belongs to the family of consensus protocols. This protocol was introduced by Leslie Lamport in 1989, named after a fictional legislative consensus system used on the Paxos island in Greece.

In this protocol, the order of execution of instruction is maintained in all the participating nodes so that all the replicas finally converge to the same value. When a client submits a request to all the participating node, it gets back a promise that the node would not accept any request with the previous timestamp.

So, when the node fails, it does not matter - the client can generate a higher value timestamp and submit to the node when it is back online. This is for a single Paxos request.

Paxos was also designed to handle multiple transaction, or client requests - but was later redesigned to Raft protocol.

> Source: https://en.wikipedia.org/wiki/Paxos_(computer_science)

## Introduction to Paxos (8 mins)

## Basic Paxos Algorithm (10 mins)
