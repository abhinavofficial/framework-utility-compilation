# AWS DynamoDB


## What is DynamoDB

* Fully Managed - No installation, upgrade, capacity planning, or manual provisioning needed
* NoSQL data model - Key-value and document models supported, in-build memory caching, support for multiple complex data types
* Access based optimization - Partition and Sort keys for efficient single or range pickup
* Scalable - Auto-scalable with simple rules based on size and read/write load. This however comes with cost and maintenance overhead.
* Redundant - No single point of failure, multi-region and thereby ensures durability
* Secure transmission, Encrypted at rest, Backup/Restore
* Ingress - Simple ingestion setup from IoT Core and other services
* Egress - Base for batch processing and analytics services, item-level change streaming to Kinesis

## Salient Features

* Tables - DynamoDB stores data (Structured Data) in tables
* Items - Each table contains multiple items. An item is a group of attributes that is uniquely identifiable among all of the other items (yes like rows)
* Attribute - is a fundamental data element, something that does not need to be broken down any further (like columns). It is usually scalar (single values). It can also support nested attribute upto 32 level deep (nested JSON is a good way to visualize)
* Allows low latency read and write access to items ranges from 1 byte up to 400KN (Key & Value put together). This is because the data is actually saved as a hash map.
* Stored on SSD which ensure high performance, spread across 3 different distinct availability zones (DC) (highly resilient, availability)
* Query Driven or Access Pattern Based Design principles. Please see the notes.
* Supports Eventual consistent reads / Strongly consistent reads (app needs to decide)
* Can be expensive for writes but not so for reads (must see "capacity units" at the time of creating a table)
* Also, supports auto scale to handle variable demand
* Can have a downloadable version that helps in development without the need for cloud services, hence reduced development costs
* Primary Key - uniquely identifies each item in the table (must be scalar) is composed of
  * Partition Key (PK) or Row Key (RK) - hashed to find out which partition the data will be stored. This should be designed not only to handle volume, but also reads and writes.
  * Sort Key (SK) - items with the same partition key are stored together, in sorted order by sort key value
* Other than primary key, we can also use indexes to fetch the date. There are two supported index - Global and Local Index. Global Index is where partition key and sort key are different which can be created anytime. Global Index is where partition key is same and sort key are different which can be created during create table.
  * There can be at maximum, 5 global and 5 local indexes.
  * In DynamoDB, you can query from secondary Index table which contains projected columns from the base table and thereby improving reads from this table.
  * Insert may take slight hit on performance since it has to maintain sync of secondary and base table.
* Each partition defines how much data would it store, how much reads and writes per second it can support (_Confirm_). If the requirement goes beyond this, a new partition can be added and the data can split. Sometimes a particular partition (or set of partitions) can be more heavily utilized (reads or writes) than others and become hotspots. We must remediate as far as possible.

> In DynamoDB, we also need to work backward. Understand the use case and plan the inserts and fetches from table to suit the requirement.

### Capacity Unit

DynamoDB read requests can be either strongly consistent, eventually consistent, or transactional.

* A _strongly consistent_ read request of up to 4 KB requires one read request unit.
* An _eventually consistent_ read request of up to 4 KB requires one-half read request unit.
* A **transactional** read request of up to 4 KB requires two read request units.

For provisioned mode tables, you specify throughput capacity in terms of read capacity units (RCUs) and write capacity units (WCUs):

* **One read capacity unit** represents one strongly consistent read per second, or two eventually consistent reads per second, for an item up to 4 KB in size. Transactional read requests require two read capacity units to perform one read per second for items up to 4 KB. If you need to read an item that is larger than 4 KB, DynamoDB must consume additional read capacity units. The total number of read capacity units required depends on the item size, and whether you want an eventually consistent or strongly consistent read. For example, if your item size is 8 KB, you require 2 read capacity units to sustain one strongly consistent read per second, 1 read capacity unit if you choose eventually consistent reads, or 4 read capacity units for a transactional read request.
* **One write capacity unit** represents one write per second for an item up to 1 KB in size. If you need to write an item that is larger than 1 KB, DynamoDB must consume additional write capacity units. Transactional write requests require 2 write capacity units to perform one write per second for items up to 1 KB. The total number of write capacity units required depends on the item size. For example, if your item size is 2 KB, you require 2 write capacity units to sustain one write request per second or 4 write capacity units for a transactional write request.

Manual -
Auto - You can specify to increase RCU / WRU if it hits the 80% / 60% of current RCU / WRU.

When RCU or WRU increases, the partition begins to split internally.

Autoscaling should be used when there is gradual increase or decrease of capacity else you would run into the constant split and compaction of partitions. In fluctuating workload, calculate the peak usage (using some metrics), and then setting initial RCU and WRU manually.

## DynamoDB Streams

* DynamoDB Streams is an optional feature that captures data modification events in DynamoDB tables (think of CDC)
* If enabled
  * If a new item is added to the table, the stream captures an image of the entire team, including all of its attributes
  * If an item is updates, the stream captures the "before" and "after" image of any attributes that were modified in the item
  * If an item is deleted from the table, the stream captures an image of the entire item before it was deleted
* Stream records have a lifetime of 24 hours; after that, they are automatically removed from the stream
* You can use DynamoDB Streams together with AWS Lambda to create a trigger - code that executes automatically whenever an event of interest appears in a stream
  * If a new customer is created then the Lambda can use SES to send a welcome email

## Global Tables

* Global tables are a managed solution for deploying a multi-region, multi-master database, without having to build and maintain your own replication solution
* Ideal for massively scaled applications, with globally dispersed users
* It is a collection of one or more replica tables, all owned by a single AWS account
* Each replica stores the same set of data items. Any given global table can have one replica table per region.
* Global tables cannot ensure immediate consistency across multi-region.
* You cannot selectively replicate the data attributes across regions - all or none is the only supported option.

## Best Practices

This section is updated per [**AWS Best Practices for DynamoDB**](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/best-practices.html)

### Key Concepts for NoSQL Design

* By contrast, you shouldn't start designing your schema for DynamoDB until you know the questions it will need to answer. Understanding the business problems and the application use cases up front is essential.
* You should maintain as few tables as possible in a DynamoDB application. Having fewer tables keeps things more scalable, requires less permissions management, and reduces overhead for your DynamoDB application. It can also help keep backup costs lower overall.

### Best Practices for designing and using partition keys effectively

Generally speaking, you should design your application for uniform activity across all logical partition keys in the table and its secondary indexes. You can determine the access patterns that your application requires, and read and write units that each table and secondary index requires.

By default, every partition in the table will strive to deliver the full capacity of 3,000 RCU and 1,000 WCU. The total throughput across all partitions in the table may be constrained by the provisioned throughput in provisioned mode, or by the table level throughput limit in on-demand mode.

DynamoDB provides some flexibility for your throughput provisioning with burst capacity. DynamoDB currently retains up to 5 minutes (300 seconds) of unused read and write capacity. During an occasional burst of read or write activity, these extra capacity units can be consumed quickly—even faster than the per-second provisioned throughput capacity that you've defined for your table.

### Best practices for using sort keys to organize data

In an Amazon DynamoDB table, the primary key that uniquely identifies each item in the table can be composed not only of a partition key, but also of a sort key.

Well-designed sort keys have two key benefits:

* They gather related information together in one place where it can be queried efficiently. Careful design of the sort key lets you retrieve commonly needed groups of related items using range queries with operators such as ```begins_with```, ```between```, ```>```, ```<```, and so on.
* Composite sort keys let you define hierarchical (one-to-many) relationships in your data that you can query at any level of the hierarchy. For example, in a table listing geographical locations, you might structure the sort key as follows.```[country]#[region]#[state]#[county]#[city]#[neighborhood]```. This would let you make efficient range queries for a list of locations at any one of these levels of aggregation, from ```country```, to a ```neighborhood```, and everything in between.

### Best practices for using secondary indexes in DynamoDB

* **Keep the number of indexes to a minimum**. Don't create secondary indexes on attributes that you don't query often. Indexes that are seldom used contribute to increased storage and I/O costs without improving application performance.
* **Choose projections carefully**. Because secondary indexes consume storage and provisioned throughput, you should keep the size of the index as small as possible. Also, the smaller the index, the greater the performance advantage compared to querying the full table. If your queries usually return only a small subset of attributes, and the total size of those attributes is much smaller than the whole item, project only the attributes that you regularly request.
* **Optimize frequent queries to avoid fetches**. To get the fastest queries with the lowest possible latency, project all the attributes that you expect those queries to return. In particular, if you query a local secondary index for attributes that are not projected, DynamoDB automatically fetches those attributes from the table, which requires reading the entire item from the table. This introduces latency and additional I/O operations that you can avoid.
* **Be aware of item-collection size limits when creating local secondary indexes**. An item collection is all the items in a table and its local secondary indexes that have the same partition key. No item collection can exceed 10 GB, so it's possible to run out of space for a particular partition key value.
* **Take advantage of sparse indexes**. For any item in a table, DynamoDB writes a corresponding index entry **only if the index sort key value is present in the item**. If the sort key doesn't appear in every table item, the index is said to be sparse. Sparse indexes are useful for queries over a small subsection of a table.

## Data Types

DynamoDB is schema. So, you can put a key and anything else in DynamoDB. However, it would be good to understand DynamoDB Data Types

| Java type        | DynamoDB type                                                                    |
|------------------|----------------------------------------------------------------------------------|
| All number types | N (number type)                                                                  |
| Strings          | S (string type)                                                                  |
| Boolean          | BOOL (Boolean type), 0 or 1.                                                     |
| ByteBuffer       | B (binary type)                                                                  |
| Date             | S (string type). The Date values are stored as ISO-8601 formatted strings.       |
| Set              | collection types- SS(string set)type, NS(number set)type, or BS(binary set)type. |

## Additional features and notes

### Transaction Support

DynamoDB has support of transactions, however, we need to understand the significant overheads of transactions in a massively scalable distributed system. Therefore, even though the transaction support is available it has certain limitations, eg the total number of entities participating in the transaction is significantly less than that of a classical relational database. You can read more in this article.

[More](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/transactions.html)

### Auto-scale Capability

As with any system, DynamoDB is also evolving and constantly getting better and feature rich. A significant pain point in any self managed NoSQL is that it needs constant monitoring to help us manage and also take a decision at some point to add more capacity. Autoscaling is one of the many powerful feature of DynamoDB. This allows automatic scaling of the infrastructure depending on the system load. However, adding more nodes (AWS will do that) will mean redistribution of data and that can add more stress to the system. Hence, we should not be adding and removing capacity at a rapid rate. You can read more in this article.

[More](https://aws.amazon.com/blogs/aws/new-auto-scaling-for-amazon-dynamodb/)

### Query Driven Design Impact

#### What will happen if we issue many queries for a given partition?

Each partition (physical DynamoDB instance) on a DynamoDB table is subject to a hard limit of write capacity units and  read capacity units. If your workload is unevenly distributed across partitions, or if the workload relies on short periods of time with high usage (a burst of read or write activity), the table might be throttled.

[DynamoDB adaptive capacity](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-partition-key-design.html#bp-partition-key-partitions-adaptive) automatically boosts throughput capacity to high-traffic partitions. However, each partition is still subject to the hard limit. This means that adaptive capacity can't solve larger issues with your table or partition (partition key) design.

To avoid hot partitions (DynamoDB instance) and throttling, optimize your table and partition (partition key) structure.

[More](https://aws.amazon.com/premiumsupport/knowledge-center/dynamodb-table-throttled/)

#### PartiQL (Open source)

It is a SQL-compatible access to relational, semi-structured, and nested data. [Read more here]( https://partiql.org/)

DynamoDB supports PartiQL, a SQL-compatible query language, to select, insert, update, and delete data in DynamoDB. Using PartiQL, you can easily interact with DynamoDB tables and run ad hoc queries using the AWS Management Console, AWS CLI, and DynamoDB APIs for PartiQL.

Due to this feature programmers can now work with DynamoDB data using SQL which brings it syntactically closer to the RDBMS. A short exercise by AWS can be found [here](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-gettingstarted.html)

#### Data export to S3

Using DynamoDB table export, you can export data from a DynamoDB table from any time within your point-in-time recovery window to an S3 bucket. Exporting a DynamoDB table to an S3 bucket enables you to perform analytics and complex queries on your data using other AWS services such as Athena, AWS Glue, and Lake Formation. DynamoDB table export is a fully managed solution for exporting DynamoDB tables at scale, and is much faster than other workarounds involving table scans.

Exporting a table does not consume read capacity on the table, and has no impact on table performance and availability. You can export table data to an S3 bucket owned by another AWS account, and to a different region than the one your table is in. Your data is always encrypted end-to-end. A short exercise by AWS can be found [here](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataExport.Requesting.html)

[AWS Doc on DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html)
