# Distributed System Architecture

A distributed system is **a set of independent processes that work together, and appear to be a single coherent system**. It has:
* No shared memory
* No common global clock (Each node is using its local clock and synchronization can become an issue and hence notion of time become very important in distributed systems)
* No shared OS

Processes interact with each other using messages over the network. There is a possibility of shared memory distributed systems, but we will ignore it for now. For us, the abstraction is a node which is a computing system with storage which is connected to another node via network.

## Core Idea
There can be many systems running a function, say database. Client request some data - it does not know where the data is. It is the job of **distributed middleware** to manage these complexities internally and should return the data to client transparently. Middleware can use any of the many protocols available (message parsing protocol) to manage internal communication.

There are several kinds of middlewares, providing several kind of abstraction for the end user.
Apache Kafka manages data in multiple nodes.
HBase manages data in distributed databases. It is an open source implementation on top of HDFS.

## Why Distributed Systems?

* Scale
  * Overcomes limitations to follow Moore's Law
  * Horizontal scaling
  * Example: Google Search indexing
* Collaboration
  * Social networks
  * eCommerce
  * Online Games
* Mobility
  * Internet of Things (IoT)

Enables Cloud Systems.

## AWS Elastic MapReduce

## Distributed System Clusters

## Objectives