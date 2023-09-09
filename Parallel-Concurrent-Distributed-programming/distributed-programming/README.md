# Distributed Systems

* Process Crash
  * It can take down for routine maintenance by system admins
  * It can be killed because some exception is not properly handled.
  * In cloud environment, it can be even trickier, as some unrelated events can bring down the servers down.
* 

* [Distributed Architecture Pattern](distributed-architecture-pattern.md)
* [Distributed Systems Architecture](distributed-systems-architecture.md)
* [Distributed Time](distributed-time.md)
* [Transaction Concurrency Control](transaction-concurrency-control.md)
* [Commit Protocols and Failure Recovery](commit-protocols-failure-recovery.md)
* [CAP Theorem](CAP-Theorem.md)
* [Consistency Models](consistency-models.md)
* [Read-Write-Quorums](read-write-quoroms.md)

## More References:
### Books
1. Pattern-Oriented Software Architecture, Volume 4, A Pattern Language for Distributed Computing
2. Using Docker: Developing and Deploying Software with Containers
3. Distributed Systems, Principles & Paradigms, 2nd Edition: Andrew S Tanenbaum & Maarten van Steen.
4. Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems: Martin Kleppmann

### Links
* [EMR](https://aws.amazon.com/emr/)
* [Clock](https://www.mdpi.com/1424-8220/20/20/5928/htm)
* [Logical Clock](https://wintermade.it/blog/posts/logical-clocks-lamport-timestamps.html)
* [Vector Clock](https://en.wikipedia.org/wiki/Vector_clock)
* [Brief Intro to Distributed Systems](https://link.springer.com/article/10.1007/s00607-016-0508-7)
* [Apache Kafka](https://kafka.apache.org/intro)
* [Docker](https://www.docker.com/)
* [Microservices](https://martinfowler.com/articles/microservices.html)


## FAQ

### This could be naive question. Where can we load such huge data for processing?

OLTP Datastore typically will use data structures like B+trees to avoid loading all the data. But for batch processing in Big data you will need to load all the data. As data grows, and more and more requests need to manipulate data.. the disk bandwidth will start coming in the way even for OLTP data stores

### Would a system with two servers fit definition of distributed system?

Well yes, with limitations which we will see shortly

### with this write ahead log solution, how any immediate read queries would be handled ?

Reads won't see the value unless it's available in the kvstore exposed to clients, the hashmap in our example. The reads which are concurrent with writes will either see those writes or will not see those writes

### have one quick question , in grand scheme of things , 1) Adding it to file will delay the put operation a bit provided we scale to millions of request (so are we trade off with latency in order to have durability) 2) What will happen if the local store crashes ?

Yes the delay is to write to only one file which is append only. That can be optimized a bit by making it an async operation as we will see. But this is a compromise to give durability guarantee by only adding the cost of single append operation

### Is the WAL cleaned up? If yes when.

Yes it's cleaned up as explained here https://martinfowler.com/articles/patterns-of-distributed-systems/low-watermark.html

### Is possible please explain WAL snapshotting with an example? Also which ACID properties does WAL provide?

You can refer to this. https://martinfowler.com/articles/patterns-of-distributed-systems/low-watermark.html

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
