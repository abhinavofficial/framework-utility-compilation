# NoSQL Database

NoSQL databases (aka "not only SQL") are non-tabular databases and store data differently than relational tables.

## Need for NoSQL

Aided by the fact that cost of storage drastically decreased in late 2000s.

Let's understand the limitation of RDBMS to get to understand the need for NoSQL

### Need for Flexible schema

RDBMS offered a very rigid and Opinionated Schema which turned out
* Suboptimal for some use cases: Hierarchical, Geospatial, Graph, Semi-Structured, Unstructured
* Schema updates are harder, might require downtime

With advent of internet, Big Data became more relevant and requires
* Large amount of structured, semi-structured and polymorphic data
* Better concurrency handling
* Flexible schema

With Agile picking up pace, the need to rapidly adapt to changing environment further generated the need for flexible schema.

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

| Criteria               | RDBMS                    | NoSQL                                                                                                                                                                                                                                                           |
|------------------------|--------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Schemas                | Structured and organized | Semi-Structured and Flexible Data                                                                                                                                                                                                                               |
| Interacting Language   | SQL                      | Not only SQL                                                                                                                                                                                                                                                    |
| Data Storage Model     | Record and Relationships | No fixed schema                                                                                                                                                                                                                                                 |
| CAP Alignment          | Consistent & available   | Availability & Partition Tolerance                                                                                                                                                                                                                              |
| Transaction Properties | ACID Properties          | BASE Transaction                                                                                                                                                                                                                                                |
| Scaling                | Vertical Scaling         | Horizontal scaling                                                                                                                                                                                                                                              |
| Failure points         | Single point of Failure  | Distributed by nature                                                                                                                                                                                                                                           |
| Joins                  | Typically required       | Typically not required                                                                                                                                                                                                                                          |
| Data to Object Mapping | Requires ORM             | Typically not required                                                                                                                                                                                                                                          |
| Purpose                | General Purpose ORM      | **Document**: general purpose </br> **Key-value**: large amounts of data with simple lookup queries </br> **Wide-column**: large amounts of data with predictable query patterns </br> **Graph**: analyzing and traversing relationships between connected data |

## Types of NoSQL Databases

Broadly classified in 4 types:
* Key-Value: 2 Column Database
* Document: Documents like json, bson, etc. and linkage between these documents.
* Wide Column: Column could be defined on its own - telemetry or time series data, etc.
* Graph: Interconnected pieces of information

## Key Value Database

The data is stored in a “key-value” format and optimized for reading and writing that data. The data is fetched by a unique key or a number of unique keys to retrieve the associated value with each key. The values can be simple data types like strings and numbers or complex objects.

In its simplest form, a key-value store is like a dictionary/array/map object as it exists in most programming paradigms, but which is stored in a persistent way and managed by a Database Management System (DBMS). Key-value databases use compact, efficient index structures to be able to quickly and reliably locate a value by its key, making them ideal for systems that need to be able to find and retrieve data in constant time.

* In-Memory Database (especially for cache implementation)
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

### Pinterest - MySQL Sharding

* Very simple sharding process based on internal ids
* Many more virtual shards than servers - easy redistribution (uses multi-master model)
* Co-locate specific user related data on one shard
* Scale: 10+ billion page views, ~100 million users

MySQL is very stable - have never lost data.

### Netflix - Apache Cassandra

* Cache storage engine
  * Subscriber data
  * Video Metadata
  * Pause Locations
  * Ratings & Recommendations
* Write heavy workloads
* Scale: 10k instances, 6PB data, 10+ million QPS

Tunability, Replicability - Cassandra

### Neo4J - Multiple use cases

* eBay - Knowledge graph for AI chat-bot
  * Facilitates inferences and transfer learning (in probabilistic manner)
  * Scale: 500 millions nodes, 20 billion relationships
* Airbnb - Data management and analytics
  * Sinks data from multiple data sources including Apache Hive
  * Common graph UI across the whole dataset - Better search and analysis
* Novartis - Knowledge graph for drug discovery
  * Graph view of compounds, proteins, diseases, genes, etc.
  * Actionable relationships populated by text mining scientific papers
  * Automated discovery of strongly correlated nodes

