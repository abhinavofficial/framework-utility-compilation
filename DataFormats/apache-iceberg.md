
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
