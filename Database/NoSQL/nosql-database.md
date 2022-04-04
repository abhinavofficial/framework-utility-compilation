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

Wide column data, also known as column family databases.

* Hybrid of NoSQL & Relational DB
  * Handles varied data types
  * Efficient writes
* Column based storage
  * Organize related facts into column families
  * Multidimensional mapping
  * Better access patterns
* Examples: Apache Cassandra, Google BigTable, Apache Hbase
* Use cases:
  * IoT devices telemetry and logs
  * Time series data
  * Transaction logs, Clickstream data
  * Heavy write & straightforward read applications
* Cassandra
  * Cassandra Query Language: Keyspace, Table, Partition, Row, Column
  * Data types: Native, Collections, Tuples, User Defined types
  * Consistent Hashing, Virtual Nodes
  * Replication Strategies, Tunable Consistency, Quorums

Consistent Hashing allows addition or removal of nodes without reorganizing data. In Cassandra, every node is a virtual node. Tunable Consistency allows you to define how many replicated writes minimally need to be successful before we can declare the system consistent. Strongest consistency that I get is when I allow Quorums of (n/2)+1 nodes.

```shell
# pull latest image of cassandra - it is 4.0.3 for not
docker pull cassandra:latest
# Now run cassandra under cluster name cass_cluster
docker run --name cass_cluster cassandra:latest
# start interactive session with csqlsh
docker exec -it cass_cluster cqlsh
```

## Document Database

* Documents (json, bson, etc.) instead of rows/records
  * Pair key with document
  * Leverage key-value databases
  * Hierarchical key-value pairs
* Flexible schema
  * Flexible field types across documents
  * Easy extensibility
* Better match for object models
* Examples: MongoDB, Apache CouchDB, RethinkDB
* Use cases:
  * Standard content management - RDBMS replacement
  * Flexible grouping like product catalogs, IoT devices
  * Aggregate Analytics
* MongoDB
  * Data types: Standard RDBMS types, GeoData, Arbitrary JSON
  * Collections, Relationships, Indexes
  * Tools: MongoDB shell, MongoDB compass, MongoDB VS

## Graph Database

* Main entities
  * Node - record/data
  * Edge/Relationship - Specific connection between nodes
  * Properties - Additional info on nodes
* Fast traversal
  * Quick path searches for network type queries
  * Easy updates on natural graph extension
* Examples: Neo4J, OrientDB, Amazon Neptune, ArangoDB
* Use Cases:
  * Social Network Graphs and Connections
  * Customer Profiles and Fraud Detection
  * Knowledge Graphs and Recommendations
  * Maps and Geospatial Graphs
  * Network Operations and Data Flow
* Neo4J
  * Nodes, Labels (to Node groups), Relationships, Properties
  * Cyber Query Language
  * Data modelling, Visualization
  * Sessions, Transactions, Queries, Causal Chaining (Bookmarks)

Causal Chaining - one query has to finish and next to start.

```shell
UserName: neo4j
Password: hkXnuUPSZG8b6Ummk_rZF-230Q26rux-MaoX5aw5qyA
```

## Selection Criteria - Which database to use?

* Available skill-set in the team
* In the beginning, start with whatever you know best.

Technical decision 
* Data Model - Structure and Flexibility
* Consistency requirements
* Availability requirements
* Concurrent load
* Scalability goals
* Querying (retrieval) capabilities and complexities
* Insertion capabilities and throughput
* Analytical and reporting capabilities
* General robustness, reliability, historical usage
* Storage and backup functionalities

### Twitter timeline in Redis

* Redis is key-value database
* Twitter uses Redis as cache for user timelines, ads, DMs, tweets
* Latest timeline available in Redis on reads
* Fan-out service in writes
  * Use adds a tweet in database
  * Asynchronous updates to timeline cache of all followers
* Scale: 100TB+ RAM, ~40MM QPS, 10k instances

###


