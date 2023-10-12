# Log Structured Merge (aka LSM) Tree

There are two data structures used in key-value stores: [B-Tree](Trees.md#b-tree) and LSM tree. If you’re using an RDBMS then you’ve most likely experienced B-tree in action. B-Tree can also be found in some key-value databases. LSM can be found in more modern relational and key-value databases. LSM is especially popular for scalable, write-intensive applications because of its speed and efficiency for writes and updates.

LSM tree is a data structure that is at the core of many more modern data and storage platforms that need write-intensive throughput. With LSM, writes are done very quickly in-memory and a transaction log (write-ahead log - WAL) is written to protect the data as it waits to be flushed from memory to persistent storage.

Speed and efficiency are increased because LSM uses an append-only write process that allows rapid sequential writes with no fragmentation challenges like B-trees are subject to. Inserts and updates can be made much faster and then the leveled tree structure is organized (and re-organized continuously) with a background compaction process to optimize key structure in the persistent Static Sort Table (SST) files.

The main disadvantage you will hear about LSM is that read performance can be poor if data is accessed in small, random chunks. Queries/reads are not done in the recursive way as a B-tree query which lends itself to good read performance. There are ways to mitigate this with the use of indexes, bloom filters, and other tuning for file sizes, block sizes, memory usage, and other tunable options.

## Pros and Cons of LSM Tree

LSM trees are especially well-suited for workloads where writes are more common than reads, or when data is accessed in large sequential chunks. LSM uses an append-only write structure which makes it super-efficient for writes. Compression and compaction help to keep data organized and reduce overall storage utilization.

### Pros of LSM

* More space efficient by using append-only writes and periodic compaction
* Better performance with fast-growing data and sequential writes
* No fragmentation penalty because of how SSTable files are written and updated

## Cons of LSM

* CPU overhead for compaction can meaningfully affect performance and efficiency if not tuned appropriately
* More tuning options increase flexibility but can seem complex to developers and operators
* Read/search performance can be optimized with the use of bloom filters

Workload patterns should define which data structure you should use, right? If the general capabilities of key-value stores using either data structure are similar it is a matter of choosing the right tradeoffs. What if we could optimize the more flexible data structure and focus on the greater of the features?
