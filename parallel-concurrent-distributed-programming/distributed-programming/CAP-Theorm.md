# CAP Theorm

Also know as Brewer's Theorm

Any **distributed data store** can only provide two of the following three guarantees:
* Consistency: Every read receives the most recent write or an error.
* Availability: Every request receives a (non-error) response, without the guarantee that it contains the most recent write.
* Partition tolerance: The system continues to operate despite an arbitrary number of messages being dropped (or delayed) by the network between nodes.

When a network partition failure happens, it must be decided whether to
* cancel the operation and thus decrease the availability but ensure consistency or to
* proceed with the operation and thus provide availability but risk inconsistency.
Thus, if there is a network partition, one has to choose between consistency and availability. Note that consistency as defined in the CAP theorem is quite different from the consistency guaranteed in ACID database transactions.

Eric Brewer argues that the often-used "two out of three" concept can be somewhat misleading because system designers only need to sacrifice consistency or availability in the presence of partitions, but that in many systems partitions are rare.

AP(Availability and Partition tolerance): When availability is chosen over consistency, the system is will always process the client request and try to return the most recent available version of the information even if it cannot guarantee it is up to date due to network partitioning.

CP(Consistency and Partition tolerance): When consistency is chosen over availability, the system will return an error or time-out if particular information cannot be updated to other nodes due to network partition or failures.

Database systems designed with traditional ACID guarantees in mind such as RDBMS choose consistency over availability, whereas systems designed around the BASE philosophy (eventual consistency), common in the NoSQL movement for example, choose availability over consistency.

The **PACELC theorem** builds on CAP by stating that in a distributed computer system even in the absence of partitioning, there is another trade-off between latency and consistency. 
```
IF network partitioning (P) THEN
  choose between availability (A) and consistency (C) (as per the CAP theorem)
ELSE (E)
  choose between latency (L) and consistency (C)
```
