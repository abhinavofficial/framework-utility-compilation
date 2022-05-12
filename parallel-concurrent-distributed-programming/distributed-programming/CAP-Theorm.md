# CAP Theorem

Also known as Brewer's Theorem

Before we move to CAP theorem, let's understand three basis concepts of distributed system. 

## Network Partitioning and Partition Tolerance

Network partition: Network split due to the failure of network devices / communication links. Assume there are 4 nodes where node 1 and 2 are connected directly while node 3 and 4 are connected directly with each other. However, these are connected amongst themselves via a network switch. In case the switch fails, both of these node sets would be operating over a split network - this is called network partition.

A network can be disrupted due to nodes going down or failure of network links. This may lead to message being dropped or lost. Nodes in partition 1 will perceive nodes in partition 2 as failed and vice versa. Nodes within each partition can elect their own leaders and behave as two different distributed systems.

We want our system to be **partition tolerant** - this means that system should continue to function properly in spite of lost messages or any other problems.

Network failures in distributed system are unavoidable. We need to tolerate network partitions

## Availability
Availability characteristics of distributed system says that the system is always available in terms of read/write request for stored data. This means that every request is met with a non-error response. However, it does not guarantee the freshness or consistency of data.

## Consistency
For every read to data in distributed system, the most recent written (and consistent) value is returned. An error is returned if we cannot find one.

Now CAP theorem states that:

Any **distributed data store** can only provide two of the following three guarantees:
* Consistency: Every read receives the most recent write or an error.
* Availability: Every request receives a (non-error) response, without the guarantee that it contains the most recent write.
* Partition tolerance: The system continues to operate despite an arbitrary number of messages being dropped (or delayed) by the network between nodes.

When a network partition happens, it must be decided whether to
* cancel the operation and thus decrease the availability but ensure consistency. Example: Apache HBase, MongoDB, Redis, **or**
* proceed with the operation and thus provide availability but risk inconsistency. Example: Apache Cassandra, Riak, CouchDB
Thus, if there is a network partition, one has to choose between consistency and availability. Note that consistency as defined in the CAP theorem is quite different from the consistency guaranteed in ACID database transactions.

> Eric Brewer argues that the often-used "two out of three" concept can be somewhat misleading because system designers only need to sacrifice consistency or availability in the presence of partitions, but that in many systems partitions are rare.

> AP(Availability and Partition tolerance): When availability is chosen over consistency, the system is will always process the client request and try to return the most recent available version of the information even if it cannot guarantee it is up to date due to network partitioning.
![](images/Availability-Prioritized.png)

> CP(Consistency and Partition tolerance): When consistency is chosen over availability, the system will return an error or time-out if particular information cannot be updated to other nodes due to network partition or failures.
![](images/Consistency_Prioritized.png)

Database systems designed with traditional ACID guarantees in mind such as RDBMS choose consistency over availability, whereas systems designed around the BASE philosophy (eventual consistency), common in the NoSQL movement for example, choose availability over consistency.

## Misconceptions related to CAP Theorem

### We always trade off between Availability and Consistency.
Fact: Both availability and consistency are feasible when no network partition occurs. Network partition happens very rarely.

### We always do without one of three guarantees
Fact: Partition tolerance is actually a given (and is managed within network topology). We only trade off between availability and consistency, when network partition occur.

## Extension of CAP Theorem (PACELC theorem)

The **PACELC theorem** builds on CAP by stating that in a distributed computer system even in the absence of partitioning, there is another trade-off between latency and consistency. 
```
IF network partitioning (P) THEN
  choose between availability (A) and consistency (C) (as per the CAP theorem)
ELSE (E)
  choose between latency (L) and consistency (C)
```


