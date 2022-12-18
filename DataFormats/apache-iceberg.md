
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
