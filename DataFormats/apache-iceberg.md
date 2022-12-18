
## Manifest file
A manifest file is a metadata file that lists a subset of data files that make up a snapshot.

Manifests track the following data points:
* **Format and Location**: Necessary for locating and reading data files after planning is complete.
* **Lower and Upper-level Bounds for Partition Values**: When query planning and determining what files to read, Iceberg uses min/max partition values to cherry-pick manifests. This ensures out-of-bounds manifests are completely skipped and our query processing engine does not waste cycles reading those files.
* **Metrics for cost-based optimizations**: Includes file-level stats (e.g. row count) and column-level stats (e.g. min value, max value). These stats help our query processing engine group tasks in an optimized manner and skip files where possible during query planning.

These additional data points enable optimizations in Iceberg such as partition pruning, file-skipping, and predicate pushdowns to the snapshot and manifest level. This coupled with columnar storage format like Parquet, makes data access significantly more lightweight and faster 

## Manifest Lists
A Manifest List is a metadata file that link manifests with the snapshots that make up a table. Each manifest file in the manifest list is stored with information about its contents, like partition value ranges, used to also speed up metadata operations and query planning. The manifest list acts as an index over the manifest files, making it possible to plan without reading all manifests.

> delegates it’s write operation to Iceberg’s DataFrame writer, which not only writes the JSON data to disk (compressed and partitioned) but also a) writes table metadata (snapshots, manifests, and manifest lists) and b) commits both the data and metadata to Data Lake in an atomic transaction.

## Concurrency Control
Iceberg employs an elegant and proven design to these problems, Multi-Version Concurrency Control (MVCC). Producers create table metadata files optimistically, assuming that the current version will not be changed before the Producer’s commit. Once a Producer has created an update, it commits by swapping the table’s metadata file pointer from the base version to the new version. Readers use the snapshot that was current when they load the table metadata and are not affected by changes until they refresh and pick up a new metadata location. Iceberg’s MVCC enables Data Platform to achieve serializable isolation in a distributed environment, where each mutation is treated as if it was executed serially and readers are not impacted by changes until they refresh.

Table metadata is stored as JSON. Each table metadata change creates a new table metadata file that is committed by an atomic operation. This operation is used to ensure that a new version of table metadata replaces the version on which it was based. This produces a linear history of table versions and ensures that concurrent writes are not lost.

The atomic operation used to commit metadata depends on how tables are tracked and is not standardized by Iceberg. However, there is a default implementation for File System tables stored on HDFS like backends such as AWS (relies on the rename operation in AWS, which is guaranteed to be atomic if overwrite is set to true).

## Time-Travel and Incremental Reads
Beyond the reliability features described above Iceberg also brings a number of useful features such as Time Travel and Incremental Reads, enabled through its use and management of table metadata. Time-Travel is the ability to make a query reproducible at a given snapshot and/or time. Incremental Reads is the ability to query what has changed between two snapshots and/or times.

### Querying By Snapshot (i.e. Time Travel)
Querying a specific version of the table:
```
SELECT * FROM dataset VERSION AS OF {SNAPSHOT_ID}
```
Querying a specific version of the table:
```
SELECT * FROM dataset VERSION AS OF {ISO_8601_DATETIME}
```
### Querying for Deltas (i.e. Incremental Reads)
Querying table for changes between snapshot versions:
```
SELECT * FROM dataset VERSION BETWEEN {START_SNAPSHOT_ID} AND {END_SNAPSHOT_ID}
```
Querying table for changes between start snapshot version to the current latest version:
```
SELECT * FROM dataset VERSION SINCE {START_SNAPSHOT_ID}
```
Querying table for changes that arrived between timestamps:
```
SELECT * FROM dataset VERSION BETWEEN {START_ISO_8601_DATETIME} AND {END_ISO_8601_DATETIME}
```
Querying table for changes that arrived after TIMESTAMP:
```
SELECT * FROM dataset VERSION SINCE {START_ISO_8601_DATETIME}
```
The above syntax shows types of queries that can now be expressed with DataSets backed by Iceberg tables.

> Note: The syntax for both Time Travel and Incremental Read features allows for either Timestamp or Snapshot based predicates. Due to the distributed nature of writes, there is a risk of time drift. As a result, the use of Snapshot Ids is best practice when accuracy is a requirement.
> It is important to point out that the Incremental query should ideally return rows that have been deleted along with what has been inserted and/or upserted. An additional metadata field (e.g. changeType), would be needed to signal what rows were deletes vs. upserts vs. inserts. _The ability to incrementally read deleted vs. updated or inserted data is not there yet but is on the roadmap._

Now lets focus on next key update, that is High Throughput Ingestion with Iceberg

## High Throughput Ingestion with Iceberg

Key agenda:
* **High Throughput Ingestion with Iceberg**: Discusses write challenges such as managing high throughput with high-frequency small files.
* **Taking Query Optimizations to the Next Level with Iceberg**: Discusses read challenges such as attaining vectorized reads.
* **Reliable Data Restatement with Iceberg**: Discusses data-restatement challenges to support General Data Protection Regulation (GDPR) like use-cases.
* **Operationalizing Iceberg**: War stories related to migrating and running our workloads at scale on-top of Iceberg.

The two technical problems that arise in a high throughput file ingestion environment:
* Small Files Problem
* High concurrent writes leading to race conditions

Iceberg helps on the read side by minimizing file scanning and more accurately locating the files that need to be loaded. When the Iceberg reader is used the data files are pruned with partition and column-level stats, using table metadata.

**Buffered writes** is a batch ingestion pattern to address our data needs since it overcomes our main two problems that arise in a high throughput file ingestion environment as Adobe Experience Platform:

Known Small Files Problem in Hadoop Distributed File System
* High concurrent writes on Iceberg tables
* This solution represents a separate service that offers a buffering point, responsible for determining when and how to package and move data from this buffer point to the data lake.

The benefits of this service are:

* **Optimizes the writes**: Less writes with a larger amount of data.
* **Optimizes the reads**: Readers will have a smaller number of files to open.
* **Auto-scaling**: Inherently available because the buffered writes use separate on-demand jobs.

One major component of the buffered writes implementation is the “consolidation worker”. This is the short-lived process that is triggered when enough data is buffered and it must be written in the main storage.

### Write Audit Publish flow (WAP)
Iceberg has the Write Audit Publish functionality that gives the possibility to store the amount of data as a staged commit later on cherry-picking it up and making it available in the table. WAP functionality relies on a specific outside given id — called wapID— by which the staged commit can be later retrieved. The most important aspect is the fact that assures uniqueness of staged commits — ensuring there cannot be two-staged commits with the same given ID.

WAP workflow is implemented in Consolidation Worker:
* Check if the data is already present in Iceberg by provided id and if so just update the high-level metadata
* Check if the data is staged as a separate commit and cherry-pick it into the table making it available to customers
* If not present in either of the above cases load the data and write it using the WAP functionality: stage the data with a specific id then cherry-pick it.
* At the end update the high-level metadata store.

### Iceberg’s version-hint.txt file improvement
An issue that Iceberg has with high throughput writes is properly resolving the latest table version to operate on. In some cases, the metafile (version-hint.txt) that provides Iceberg’s SDK the current version goes missing due to a known non-atomic rename/move operation.

When Iceberg commits a new version it will do so by using the HDFS CREATE (overwrite=true) API to replace the current content of version-hint.text with the new version value (an increment of the current one).

Internally AWS chose to implement this API as a DELETE + CREATE operation instead. This creates the possibility for a particular window of time, on each commit operation, where there is no version-hint.text file available.

This raises inconsistency since both the Iceberg reader and writer to depend on the version-hint file to resolve the version and select the appropriate metadata version files to load.

The solution relies on moving to a different implementation for persisting table version by leveraging the ADLS filesystem APIs guarantees — such as atomicity of CREATE(overwrite=false) and read-after-write consistency of list directory.

Hence the implementation can be to _switch to preserving version using directory listing instead, so each writer will use CREATE(overwrite-=false) to create a new file to signal the new version while readers will have to list the versions directory and pick the highest value present at that particular moment in time_.

**Iceberg Community may address and fix it differently**.
