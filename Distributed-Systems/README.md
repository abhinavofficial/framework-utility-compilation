# Distributed Systems

* [Fundamentals](fundamentals.md)
* [Distributed Time](distributed-time.md)
* [Transaction Concurrency Control](transaction-concurrency-control.md)
* [Commit Protocols and Failure Recovery](commit-protocols-failure-recovery.md)
* [CAP Theorem](CAP-Theorem.md)
* [Consistency Models](consistency-models.md)
* [Read-Write-Quorums](read-write-quoroms.md)
* [Distributed Architecture Pattern](architecture-pattern.md)
* [Data Replication](data-replication.md)

## More References

### Books

1. Pattern-Oriented Software Architecture, Volume 4, A Pattern Language for Distributed Computing
2. Using Docker: Developing and Deploying Software with Containers
3. Distributed Systems, Principles & Paradigms, 2nd Edition: Andrew S Tanenbaum & Maarten van Steen.
4. Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems: Martin Kleppmann

### Links

* [EMR](https://aws.amazon.com/emr/)
* [Brief Intro to Distributed Systems](https://link.springer.com/article/10.1007/s00607-016-0508-7)
* [Apache Kafka](https://kafka.apache.org/intro)
* [Docker](https://www.docker.com/)
* [Microservices](https://martinfowler.com/articles/microservices.html)

## FAQ

### This could be naive question. Where can we load such huge data for processing?

OLTP Datastore typically will use data structures like B+trees to avoid loading all the data. But for batch processing in Big data you will need to load all the data. As data grows, and more and more requests need to manipulate data.. the disk bandwidth will start coming in the way even for OLTP data stores

### Would a system with two servers fit definition of distributed system?

Well yes, with limitations which we will see shortly

### What if, at the exact moment in time, byzantium gets back online (but didn't repair it's latest value), and cyrene (which had the correct latest value) goes offline. The value is effectively "lost" until we get cyrene back up, right?

As long as we have athens having the latest value we are fine.. we wait only for quorum and assume that the one node with the latest value is the value from the last successful write

In quorum read do you have to wait for responses from all servers? If not then what happens if most of the responses you have till you reach the quorum mark are from bad servers?

If majority are failures, the operation fails..

### Do you need quorum in reads if using timestamps? If yes then why?

You need quorum for reads if you need ;read your own writes kind of guarantee.

### What's your advice for system design interview

Know the principles and patterns well..

### is this Quorum based techniques used in apache spark ?

Its used in data stores which need to tolerate failures and still give availability.. so the stores which spark uses for reading/writing data from do use quorum..

### does it copy the data from node1 to node2 or byzantiumread won't have any data but still say successful?

It will write data to quorum and even if some of the nodes miss data, it will be a successful write as long as majority quorum is reached

### Do these databases vendors makes these design decisions already for app developers? what level of controls do we have ?

The decisions are taken for us.. we have to know the implications particuarly related to consistency and worst case related to data loss.
