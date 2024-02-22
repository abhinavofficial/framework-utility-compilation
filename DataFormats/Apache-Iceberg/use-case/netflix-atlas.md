# Netflix - Atlas

Historical Atlas data:

* Time-series matrices from Netflix runtime systems
* 1 month: 2.7 million files in 2,688 partitions
* Problem: cannot process more than few days of data

Sample query:

```sql
select distinct tags['type'] as type 
from iceberg.atlas
where 
    name = 'metric-name' and
    date > 20180222 and date <= 20180228
order by type;
```

Output

* Hive table - with parquet filters:
  * 400k+ splits, not combined
  * Explain query: 9.6(planning wall time)

* Iceberg table - partition data filtering:
  * 15,218 splits, combined
  * 13 min (wall time) / 61.5 hr (task time) / 10 sec (planning)

* Iceberg table - partition and min/max filtering:
  * 412 splits
  * 42 sec (wall time) / 22 min (task time) / 25 sec (planning)

## Hive table

Table design: key idea: organize data in a directory tree.

Problem: too much directory listing for large tables
Solution:

* Use HMS to track parititions
  * Partition key to FS location
  * Enables predicate push-down in HMS for (some) scans
* The file system still tracks the files in each partitions

### Design Problems

* Table state is stored in two places
  * Paritions in the HMS
  * Files in a FS with **no transaction support**
* Requires elaborate locking within HMS for correctness
  * **Nothing expect Hive respects the locking scheme** so Spark, Presto, etc can get the data incorrect.
* Layout is the **opposite of S3 recommendations**
  * S3 wants to mix up data - data that you are reading and the ones you are not, so that portition of the S3 index that you are reading are on separate nodes on their S3 architecture to increase parallelism.
* Still requires directory listing to plan jobs
  * **O(n) listing call where n is number of matching partitions**
  * **Eventual consistency breaks correctness** - this is now solved.
* Partition values are stored as strings
  * Requires character escaping
  * null stored as __HIVE_DEFAULT_PARTITION__
* HMS table metrics get stale
  * Files can be added or removed at any time
* Metastore is a scale bottleneck
  * HMS is backed by a central relational DB
* Table definition tied to Java
* Users must know and use a table's physical layout
  * ts > X => full table scan!
* Schema evolution rules are dependent on the file format.
  * Example: CSV - by position
  * Example: AVRO - by name
* Unreliable: type support varies across formats
  * Which formats support decimal?
  * Does CSV support maps with struct keys?

## Apache Icerberg
 