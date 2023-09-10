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
  * Overcomes limitations to follow [Moore's Law](moore-law.md) and [Little's Law](little-law.md)
  * Horizontal scaling
  * Example: Google Search indexing
* Collaboration
  * Social networks
  * eCommerce
  * Online Games
* Mobility
  * Internet of Things (IoT)
* To manage consistent service state with distributed State and partial failures within the system

Enables Cloud Systems.

## AWS Elastic MapReduce

Elastic MapReduce can be used for writing map-reduce programs and doing distributed analytics. While using EMR, you can create a cluster of servers. OS and middleware (Hadoop) runs on each of these servers. You can create any number of nodes - node failures are automatically taken care of. If the master (also known as name node) detects a node being failed, it replicated those portion of the data on other healthy nodes. Discover of an issue and recovering from the issue is a part of distributed systems. If the data node does not communicate with the master node, master node considers that data node failed which then triggers recovery action. Similarly, the way data consistency is maintained (in terms of replicas, etc.) is also part of distributed system. 

Hadoop Master node remains in safe mode if it is not able to meet the desired replica configuration. Data node sends heartbeat period - timeout is critical details. What kind of failures are we assuming in our systems. What if the data nodes are sending wrong information to master and other data nodes.

## Properties of Distributed Systems

* **Fault-Tolerance**: Specify replication factor of 3 (the number of replicas that master will manage is 3). Data node sends heart-beat message.

* **Consistency**: Updates on Data Node 1 must be replicated to Data Node 2. Updates in HDFS is an append operation where the data blocks are appended at the end of existing HDFS block. One way to manage consistency is by using **at-least once semantics** (easily implementable) or **at-most once semantics** or **exactly once semantics** (difficult to implement).

## Objectives

* Scalability - Add as many nodes are required to support the process
  * Conventional Systems: Vertical Scaling
    * More memory, processing power, network bandwidth, in one server
    * Limited by physical machine size
  * Distributed System: Horizontal scaling (based on some saturation or threshold point)
    * Increase number of individual units
      * Compute servers
      * Database server nodes
  * Advantages
    * Can scale on-demand
    * Less expensive than vertical scaling
    * No single point of failure
* Reliability - If some node is down, recover transparently
  * Handle failure by adding redundancy
  * If components fail, adapt to handle requests
  * detect failures to address them
* Availability - If some node is down, system continues to run transparently
  * The fraction of time, a system is usable
  * Can achieve with redundancy - copies of data
  * Challenge: Maintaining data consistency
* Efficiency - For example, HDFS is optimized for reads instead of writes.
* Serviceability - System should be able to discover the fault and automatically recover if possible.
* Manageability
  * Load Balancing mechanism to optimize the usage

CPU
RAM
Network
Disk

Problems that we write on the distributed system.

* **Process Crash**
  * It can be take down for routine maintenance by system admins
  * It can be killed because some exception is not properly handled.
  * In cloud environment, it can be even trickier, as some unrelated events can bring down the servers down.
* **Unsynchronized Clocks**
  * Time of the day can drift at different rates
  * Adjustments done with the NTP can cause the clocks to go "back in time"
  * Clocks from different servers cannot be compared.
* **Network Delays**
  * TCP/IP is by design 'asynchronous'.
  * There is no upper bound on the network delays.
  * A server cannot wait indefinitely to know if another server has failed.
  * There should not be two sets of servers serving clients, each considering the other has failed.
* **Process Pause**
  * Garbage collection pause.
  * A process is not scheduled by the scheduled by the scheduler.
  * The process is not aware that it was paused.

These problems can cause -

* Loss of data in memory.
* Inconsistent data state
  
We need to give some guarantee to consumers.