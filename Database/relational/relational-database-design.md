# Database Design

## History

* 1950 - 1960s
  * Punched cards and magnetic tapes
  * Sequential data
  * Procedural ways
* Late 1960s - 1970
  * Hard disk - storage space and access
  * Relational model proposed
  * Non-procedural way to access data
* 1980 - 2000
  * Processing still very transactional
  * Little movement to RDBMS
  * SQL was born
* 2000 and beyond
  * Advent and explosion of internet

Edgar F. Codd - "A relational Model for data for large data banks", June 1970 - invented the relational model for database management.

## Why Databases?

* Store very large number of records efficiently - optimized for space
* Find information - quickly and easily
* Add new data and modify old data - easily
* Search and sort data - easily
* Support concurrency - access the same data
* Data security and Integrity (Consistency)
* Typical structure - Table/Row/Column/Record

Large data is defined in terms of total size, rate of incoming data and rate of retrieval.

## Database Design

* Properties of a well-designed database
  * Reflects real-world structure of the problem
  * Consistent and easy to understand
  * Avoids data redundancy
  * Efficient and faster access of data
  * Ease of maintenance
  * Scales well in all aspects!
* Considerations for a good design
  * End user perspective - fast and reliable
  * Application driven - various verticals have their own demands
  * Use cases to satisfy - the right queries
  * Bridge - technology, use cases
  * Tradeoffs - Various tradeoff based on size, rate of increase
  * Performance parameters - response times!

## Design Basics - Relationships

Entity-Relationship Model
* Describes data as entities, relationships, and attributes
* Entity: A real-world thing which has independent existence. Example - Person, Car
* Attributes: Properties that describe the entity. Examples: Name, Address, Color, Brand
* Relationships: A set of associations among entity types. Example, Employee works for department, Car belongs to person
  * One-to-many relationship: One entity record can be associated with 0, 1, or many records of other entity
  * One-to-one relationship: One entity record can be associated with 0 or 1 record of other entity
  * Many-to-many relationship: 0, 1, or many records of one entity can be associated with 0, 1, or many records of other entity. It requires a junction table to establish this relationship.

## Design Basics - Keys

* Primary
  * Uniquely identifies each record in a table
  * One and only one primary key
* Alternate
  * Uniquely identifies each record in a table
  * Multiple alternate keys possible
  * Single column or group of columns
* Foreign
  * Creates relationship between two tables
  * Uniquely identifies each record in the originating table
  * Helps maintain referential integrity

## Design Basics - Normalization

### Anomalies

It can lead to problems in database table structure and may prevent correct functioning or causes inefficiencies. This generates the motivation for normalization, with obvious reason of optimal utilization of space. There are three kinds of anomalies.

* Insertion Anomalies
  * Forced to insert with null with some columns
  * Data not available at the time of insertion
* Update Anomalies
  * Forced to update same information in multiple rows
  * Data inconsistency if not done correctly
* Deletion Anomalies
  * Unwanted deletion of important data
  * Independent data stored only with other information

### Process

* Analyse
  * Functional dependencies
  * Primary/Alternate Keys
  * Foreign Keys
* Goals
  * Minimize redundancy
  * Eliminate all anomalies
  * Allow extensibility with redesign

### Techniques

* First Normal Form (1NF): Values in each column must be atomic - meaning there should be only one value in a column. Example: Comma-separate courses list in courses column in students table violates 1NF and hence, courses should be stored in a separate table and mapped to each student.
* Second Normal Form (2NF): No column dependent on partial unique set. Example: If both deptID and dept_name are stored in a students table, it would violate 2NF. Students table should have deptID ONLY.
* Third Normal Form (3NF): No transitive dependency on unique set. Example: zip_code, address_district, address_state are stored in students table. This violates 3NF. Only zip_code should be stored in students table. zip_code mapping to district and state should be stored separately.
* There are more -
  * Boyce-Codd Normal Form (BCNF): Removes all the functional dependencies.
  * Fourth normal form (4NF)
  * Fifth normal form (5NF)
  * Sixth normal form (6NF)

## Database Performance Optimization - Indexes

### Introduction
* Improves performance of database queries
* Help find relevant information without scanning the whole table
* Make query based on a column (or a set of columns) faster
* Helps in both random access and ordered set retrieval
* Additional storage cost
* Makes writes slower

### Architecture

* A set of records
  * Search Key (Single column or column set values)
  * Actual row data or pointer to the data
* Commonly used data structures
  * Balanced trees
  * B+ trees
  * Hashes
* Types
  * Clustered Index
    * Actual table data is organized based on the search key order
    * Highly efficient; only one such index possible
  * Non-clustered Index
    * Pointers to all relevant rows stored for a search key
    * Requires more block reads
    * Multiple such indexes allowed
    * Typically, I will have non-clustered data when I have joins

## Database Performance Optimization - Denormalization

* Why? To improve retrieval performance
* When?
  * Normalize first
  * Exhaust indexes and other techniques
  * Use of limited scope in hotspots
* How?
  * By reducing joins
  * By storing aggregate data
* Pitfalls
  * Inserts and updates are slower
  * Additional logic to keep redundant data in sync
  * Incorrect implementation can lead to data inconsistencies
  * Additional storage requirement
  * Reduces efficiency of the memory buffer cache

> Denormalization is a balancing act!

## Advent of multi-instance configurations

Motivation: is to build fast, reliable and scalable applications, which is measured in terms of:
* Response time
* Throughput (Quantum of data going back and forth)

This leads to evaluate scaling factors in terms of:
* Transactions per unit time
* Good response time across requests

Couple of things to note is that, for any application:
* Loads (reads or writes) would be uneven.
* Some system might require higher reliability and throughput (say, fire and alarm system), some may be okay even if you miss some data points (say, solar system) while some in between may remain inconsistent for few hours (say, fleet management system).

### Scaling - Types

Fundamentally, there are two scaling types:
* Vertical Scaling
  * Increase hardware capacity of one machine (single point of failure)
* Horizontal scaling
  * Use multiple machines to increase performance
  * Load balance: Distribute the incoming load to all machines effectively

Distribution System challenges
* Ensure data consistency
* Handle machine failures
* Provide fast recovery paths

Let's consider different kinds horizontal scaling

### Master-Slave Replication

* Multiple read instances (distributed reads)
* Read/write ratio is high
* Single Source of Truth
* Non-blocking reads / writes

Replication log will cause stale data reads from the replicas.

### Multi-Master Replication
* Read/write rati is not too high
* Need to write on more instances
* Single source of truth (?)
* Two-way replication

Async replication can be established between each master.

To implement this, we use something called **sharding** (horizontal partitioning)
* Break data into small chunks (either vertically or by range of data, horizontally)
* It allows me to have more compute capacity per server
* Manual/Automatic sharding
  * Hash
  * Range

MySql basic have manual sharding while MongoDB Atlas has automatically sharding.

When sharded, aggregating data is much harder.

More on sharding [here](../sharding.md)

## More learning

### Links
* [Normalization](https://www.essentialsql.com/database-normalization/)

### Books
* Database Management System by Johannes Gehrke and Raghu Ramakrishnan
* Fundamentals of Database System by Ramez Elmasri