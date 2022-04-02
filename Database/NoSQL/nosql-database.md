# NoSQL Database

## Need for NoSQL

Let's understand the limitation of RDBMS to get to understand the need for NoSQL
### Rigid and Opinionated Schema
* Suboptimal for some use cases: Hierarchical, Geospatial, Graph, Semi-Structured, Unstructured
* Schema updates are harder, might require downtime

With advent of internet, Big Data became more relevant and requires
* Large amount of structured and semi-structured data
* Better concurrency handling
* Flexible schema

NoSQL provides all these

### Scaling
* Vertical Scaling (Default): Expensive, Disruptive, Limited
* Horizontal Scaling: Unwieldy, Mismatched with core RDBMS principles

We need practically limitless scalability. NoSQL databases provides Inbuilt, responsive horizontal scaling.

### Single Point of Failure
RDBMS is a single place where all the data is present and certainly susceptible to failure if not replicated well.

NoSQL databases are typically distributed and hence it provides required fault tolerance and replicated data for desired resilience. It is focused on high availability.

### ACID (Atomicity, Consistency, Isolated and Durability) Compliance
* Great for specific use cases
* Imposes structural cost thereby limiting scaling options

## Properties for NoSQL

* Elasticity
  * Add or remove resources as needed
* Sharding
  * Automatic partitioning to distribute the load
  * Consistent hashing
* Asynchronous Replication
  * Store and forward approach
* Horizontal scaling
  * Scaling out
  * Commodity hardware
* Basically Available
  * Data availability by replication
  * System is always available but data may not
* Soft State
  * No mutual consistency guarantees at a given time
  * Probabilistic state convergence
* Eventual Consistency
  * Consistency after certain time, if no further updates
  * Might require reconciliation

**B**asically **A**vailable Soft **S**tate and **E**ventually Consistency is popularly referred to as BASE (against ACID). Note that there is no concept of transaction as defined in RDBMS in NoSQL.

## Comparison between Relational and NoSQL

| RDBMS                    | NoSQL                              |
|--------------------------|------------------------------------|
| Structured and organized | Semi-Structured and Flexible Data  |
| SQL                      | Not only SQL                       |
| Record and Relationships | No fixed schema                    |
| Consistent & available   | Availability & Partition Tolerance |
| ACID Properties          | BASE Transaction                   |
| Vertical Scaling         | Horizontal scaling                 |
| Single point of Failure  | Distributed by nature              |

## Types of NoSQL Databases

Broadly classified in 4 types:
* Key-Value: 2 Column Database
* Document: Documents like json, bson, etc. and linkage between these documents.
* Wide Column: Column could be defined on its own - telemetry or time series data, etc.
* Graph: Interconnected pieces of information

## Key Value Database
* In-Memory Database
  * Fixed size - drops older keys, LRU based..
  * Guaranteed expiry (deletion), if specified
* Globally distributed hash table
  * Use hash function to route data
  * Consistent Hashing: Minimizes data realignment on node additions/removals
* Commands to access data
  * get, put, delete
* Examples - Redis, Berkeley DB, MemcacheDB, Riak, Amazon DynamoDB, RocksDB
* Use cases
  * Cache Storage
  * Session Management, Configurations, and Preferences
  * Transient data like e-commerce shopping carts, and latest news feed updates
  * Derived data like counts, leaderboards
* Redis
  * Value data types: Stings, Lists, Sets, Hashes, Sort sets, Bitmaps
  * Persistence: RedisDB aka RDB (Point-in-time snapshots), Append only file aka AOF(Write logging)
  * Clustering: Master and slave nodes, Seamless scaling, Eventually consistent

This database is typically based by a NoSQL DB to manage persistence. If data is not found in memory, it fetched from this DB and makes it available.

For example, in Redis RDB or AOF serves for persistence. In RDS, frequency in which the snapshots are managed may influence performance, so careful judgement required. AOF allows reconstruction of data (from the point of snapshot).

## Wide-Column Database

