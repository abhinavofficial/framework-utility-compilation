# Sharding

## What is Sharding?
Sharding is a database architecture pattern related to **horizontal partitioning** — the practice of separating one table’s rows into multiple different tables, known as partitions. Each partition has the same schema and columns, but also entirely different rows. Likewise, the data held in each is unique and independent of the data held in other partitions.

It can be helpful to think of horizontal partitioning in terms of how it relates to vertical partitioning. In a vertically-partitioned table, entire columns are separated out and put into new, distinct tables. The data held within one vertical partition is independent of the data in all the others, and each holds both distinct rows and columns. The following diagram illustrates how a table could be partitioned both horizontally and vertically:

Sharding involves breaking up one’s data into smaller chunks, called **logical shards**. The logical shards are then distributed across separate database nodes, referred to as **physical shards**, which can hold multiple logical shards. Despite this, the data held within all the shards collectively represent an entire logical dataset.

Database shards exemplify a **shared-nothing architecture**. This means that the shards are autonomous; they don’t share any of the same data or computing resources. In some cases, though, it may make sense to replicate certain tables into each shard to serve as reference tables.

Oftentimes, sharding is implemented at the application level, meaning that the application includes code that defines which shard to transmit reads and writes to. However, some database management systems have sharding capabilities built in, allowing you to implement sharding directly at the database level.

## Benefits of Sharding
The main appeal of sharding a database is that it can help to facilitate **horizontal scaling, also known as scaling out**. Horizontal scaling is the practice of adding more machines to an existing stack in order to spread out the load and allow for more traffic and faster processing. This is often contrasted with **vertical scaling, otherwise known as scaling up**, which involves upgrading the hardware of an existing server, usually by adding more RAM or CPU.

It’s relatively simple to have a relational database running on a single machine and scale it up as necessary by upgrading its computing resources. Ultimately, though, any non-distributed database will be limited in terms of storage and compute power, so having the freedom to scale horizontally makes your setup far more flexible.

It helps to speed up query response times. By sharding one table into multiple, though, queries have to go over fewer rows and their result sets are returned much more quickly.

Sharding can also help to make an application more reliable by mitigating the impact of outages.

## Drawbacks of Sharding

The first difficulty that people encounter with sharding is the sheer complexity of properly implementing a sharded database architecture. If done incorrectly, there’s a significant risk that the sharding process can lead to lost data or corrupted tables. Even when done correctly, though, sharding is likely to have a major impact on your team’s workflows. Rather than accessing and managing one’s data from a single entry point, users must manage data across multiple shard locations, which could potentially be disruptive to some teams.

One problem that users sometimes encounter after having sharded a database is that the shards eventually become unbalanced unless we manage database hotspot effectively.

Another major drawback is that once a database has been sharded, it can be very difficult to return it to its unsharded architecture.

A final disadvantage to consider is that sharding isn’t natively supported by every database engine. Because of this, sharding often requires a “roll your own” approach. This means that documentation for sharding or tips for troubleshooting problems are often difficult to find.

## Sharding Architectures
In this section, we’ll go over a few common sharding architectures, each of which uses a slightly different process to distribute data across shards.

### Key Based Sharding
Key based sharding, also known as **hash based sharding**, involves using a value taken from newly written data and plugging it into a hash function to determine which shard the data should go to. A hash function is a function that takes as input a piece of data (for example, a customer email) and outputs a discrete value, known as a hash value. In the case of sharding, the hash value is a shard ID used to determine which shard the incoming data will be stored on.

To ensure that entries are placed in the correct shards and in a consistent manner, the values entered into the hash function should all come from the same column. This column is known as a **shard key**. In simple terms, shard keys are similar to primary keys in that both are columns which are used to establish a unique identifier for individual rows. Broadly speaking, a shard key should be static, meaning it should not contain values that might change over time. Otherwise, it would increase the amount of work that goes into update operations, and could slow down performance.

While key based sharding is a fairly common sharding architecture, it can make things tricky when trying to dynamically add or remove additional servers to a database. As you add servers, each one will need a corresponding hash value and many of your existing entries, if not all of them, will need to be remapped to their new, correct hash value and then migrated to the appropriate server. As you begin rebalancing the data, neither the new nor the old hashing functions will be valid. Consequently, your server won’t be able to write any new data during the migration and your application could be subject to downtime.

The main appeal of this strategy is that it can be used to evenly distribute data to prevent hotspots. Also, because it distributes data algorithmically, there’s no need to maintain a map of where all the data is located, as is necessary with other strategies like range or directory based sharding.

### Range Based Sharding
Range based sharding involves sharding data based on ranges of a given value of a column.

The main benefit of range based sharding is that it’s relatively simple to implement. Every shard holds a different set of data, but they all have an identical schema as one another, as well as the original database. The application code reads which range the data falls into and writes it to the corresponding shard.

On the other hand, range based sharding does not protect data from being unevenly distributed, leading to the aforementioned database hotspots. Looking at the example diagram, even if each shard holds an equal amount of data the odds are that specific products will receive more attention than others. Their respective shards will, in turn, receive a disproportionate number of reads.

### Directory Based Sharding
To implement directory based sharding, one must create and maintain a lookup table that uses a shard key to keep track of which shard holds which data. A lookup table is a table that holds a static set of information about where specific data can be found.

Here, the particular column is defined as a shard key. Data from the shard key is written to the lookup table along with whatever shard each respective row should be written to. This is similar to range based sharding, but instead of determining which range the shard key’s data falls into, each key is tied to its own specific shard. Directory based sharding is a good choice over range based sharding in cases where the shard key has a low cardinality — meaning, it has a low number of possible values — and it does not make sense for a shard to store a range of keys. Note that it’s also distinct from key based sharding in that it does not process the shard key through a hash function; it just checks the key against a lookup table to see where the data needs to be written.

The main appeal of directory based sharding is its flexibility. Range based sharding architectures limit you to specifying ranges of values, while key based one's limit you to using a fixed hash function which, as mentioned previously, can be exceedingly difficult to change later on. Directory based sharding, on the other hand, allows you to use whatever system or algorithm you want to assign data entries to shards, and it’s relatively easy to dynamically add shards using this approach.

While directory based sharding is the most flexible of the sharding methods discussed here, the need to connect to the lookup table before every query or write can have a detrimental impact on an application’s performance. Furthermore, the lookup table can become a single point of failure: if it becomes corrupted or otherwise fails, it can impact one’s ability to write new data or access their existing data.

### Should I Shard?

Because of the added complexity, sharding is usually only performed when dealing with very large amounts of data. Here are some common scenarios where it may be beneficial to shard a database:
* The amount of application data grows to exceed the storage capacity of a single database node.
* The volume of writes or reads to the database surpasses what a single node or its read replicas can handle, resulting in slowed response times or timeouts.
* The network bandwidth required by the application outpaces the bandwidth available to a single database node and any read replicas, resulting in slowed response times or timeouts.

Before sharding, you should exhaust all other options for optimizing your database. Some optimizations you might want to consider include:
* Setting up a remote database
* Implementing caching. Caching involves temporarily storing data that has already been requested in memory, allowing you to access it much more quickly later on.
* Creating one or more read replicas. Distributing reads and writes keeps any one machine from taking on too much of the load, helping to prevent slowdowns and crashes.
* Upgrading to a larger server. In most cases, scaling up one’s database server to a machine with more resources requires less effort than sharding.