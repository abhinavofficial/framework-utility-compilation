# Apache Iceberg

## Iceberg Catalog

table -> current metadata pointer

## Metadata Layer

Stored where data layer sits (S3 or HDFS)

### Manifest file

A manifest file is a metadata file that lists a subset of data files that make up a snapshot.

Manifests track the following data points:

* **Format and Location**: Necessary for locating and reading data files after planning is complete.
* **Lower and Upper-level Bounds for Partition Values**: When query planning and determining what files to read, Iceberg uses min/max partition values to cherry-pick manifests. This ensures out-of-bounds manifests are completely skipped and our query processing engine does not waste cycles reading those files.
* **Metrics for cost-based optimizations**: Includes file-level stats (e.g. row count) and column-level stats (e.g. min value, max value). These stats help our query processing engine group tasks in an optimized manner and skip files where possible during query planning.

These additional data points enable optimizations in Iceberg such as partition pruning, file-skipping, and predicate push-downs to the snapshot and manifest level. This coupled with columnar storage format like Parquet, makes data access significantly more lightweight and faster 

### Manifest Lists

A Manifest List is a metadata file that link manifests with the snapshots that make up a table. Each manifest file in the manifest list is stored with information about its contents, like partition value ranges, used to also speed up metadata operations and query planning. The manifest list acts as an index over the manifest files, making it possible to plan without reading all manifests.

> delegates it’s write operation to Iceberg’s DataFrame writer, which not only writes the JSON data to disk (compressed and partitioned) but also a) writes table metadata (snapshots, manifests, and manifest lists) and b) commits both the data and metadata to Data Lake in an atomic transaction.

### Snapshot

Table is a series for snapshots

## Data Layer

Actual data file

## Concurrency Control

Iceberg employs an elegant and proven design to these problems, Multi-Version Concurrency Control (MVCC). Producers create table metadata files optimistically, assuming that the current version will not be changed before the Producer’s commit. Once a Producer has created an update, it commits by swapping the table’s metadata file pointer from the base version to the new version. Readers use the snapshot that was current when they load the table metadata and are not affected by changes until they refresh and pick up a new metadata location. Iceberg’s MVCC enables Data Platform to achieve serializable isolation in a distributed environment, where each mutation is treated as if it was executed serially and readers are not impacted by changes until they refresh.

Table metadata is stored as JSON. Each table metadata change creates a new table metadata file that is committed by an atomic operation. This operation is used to ensure that a new version of table metadata replaces the version on which it was based. This produces a linear history of table versions and ensures that concurrent writes are not lost.

The atomic operation used to commit metadata depends on how tables are tracked and is not standardized by Iceberg. However, there is a default implementation for File System tables stored on HDFS like backends such as AWS (relies on the rename operation in AWS, which is guaranteed to be atomic if overwrite is set to true).

## Time-Travel and Incremental Reads

Beyond the reliability features described above Iceberg also brings a number of useful features such as Time Travel and Incremental Reads, enabled through its use and management of table metadata. Time-Travel is the ability to make a query reproducible at a given snapshot and/or time. Incremental Reads is the ability to query what has changed between two snapshots and/or times.

### Querying By Snapshot (i.e. Time Travel)

Querying a specific version of the table:

```sql
SELECT * FROM dataset VERSION AS OF {SNAPSHOT_ID}
```

Querying a specific version of the table:

```sql
SELECT * FROM dataset VERSION AS OF {ISO_8601_DATETIME}
```

### Querying for Deltas (i.e. Incremental Reads)

Querying table for changes between snapshot versions:

```sql
SELECT * FROM dataset VERSION BETWEEN {START_SNAPSHOT_ID} AND {END_SNAPSHOT_ID}
```

Querying table for changes between start snapshot version to the current latest version:

```sql
SELECT * FROM dataset VERSION SINCE {START_SNAPSHOT_ID}
```

Querying table for changes that arrived between timestamps:

```sql
SELECT * FROM dataset VERSION BETWEEN {START_ISO_8601_DATETIME} AND {END_ISO_8601_DATETIME}
```

Querying table for changes that arrived after TIMESTAMP:

```sql
SELECT * FROM dataset VERSION SINCE {START_ISO_8601_DATETIME}
```

The above syntax shows types of queries that can now be expressed with DataSets backed by Iceberg tables.

> Note: The syntax for both Time Travel and Incremental Read features allows for either Timestamp or Snapshot based predicates. Due to the distributed nature of writes, there is a risk of time drift. As a result, the use of Snapshot Ids is best practice when accuracy is a requirement.
> It is important to point out that the Incremental query should ideally return rows that have been deleted along with what has been inserted and/or upserted. An additional metadata field (e.g. changeType), would be needed to signal what rows were deletes vs. upserts vs. inserts. _The ability to incrementally read deleted vs. updated or inserted data is not there yet but is on the roadmap._

## Taking Query Optimizations to the Next Level with Iceberg

## Compaction: Enabled by Design

* Asynchronously compact small files into fewer larger files
* It being asynchronously helps balance the write-side and read-side trade offs
* Input and output of compaction jobs can be different file types
  * E.g. avro from streaming writes, compacted into larger parquet files for analytics
* Scheduling/trigger and the actual compaction work is done by external tools
  * Scheduling/triggering: scheduler, workflow tool, etc. - every hour of so, if required
  * Compaction work execution: processing engine (e.g. Spark, Dremio)

## Benefits of Apache Iceberg

* **Efficiently make smaller updates**
  * Make changes at the file level (128 or 256 MB)
* **Snapshot isolation for transactions**
  * Reads and writes don't interference with each other and all writes are atomic
  * Concurrent writes
* **Faster planning and execution**
  * List of files defined on the write-side
  * Column stats in manifest files used to eliminate files
* **Reliable metrics for CBO**
  * Done on write instead of "infrequent" expensive read job
* **Abstract the physical, expose a logical view**
  * Hidden partitioning
  * Compaction
  * Tables can change over time
  * Data Engineers can transparently experiment with table layout
* **Rich schema evolution support**
* **All engine see changes immediately**
* **Event listener**
  * Like trigger in rdbms
  * Cache and materialized view maintenance
  * Incremental processing
  * Future and pluggability

## Additional Resources

* [Dremio Abstract](https://dremio.com/subsurface/apache-iceberg)
* [Get hands on](https://iceberg.apace.org/getting-started)
