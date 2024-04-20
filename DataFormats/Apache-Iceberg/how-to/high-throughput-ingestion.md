# High Throughput Ingestion with Iceberg

[Source of this doc](https://blog.developer.adobe.com/high-throughput-ingestion-with-iceberg-ccf7877a413f)

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

## Write Audit Publish flow (WAP)

Iceberg has the Write Audit Publish functionality that gives the possibility to store the amount of data as a staged commit later on cherry-picking it up and making it available in the table. WAP functionality relies on a specific outside given id — called wapID— by which the staged commit can be later retrieved. The most important aspect is the fact that assures uniqueness of staged commits — ensuring there cannot be two-staged commits with the same given ID.

WAP workflow is implemented in Consolidation Worker:

* Check if the data is already present in Iceberg by provided id and if so just update the high-level metadata
* Check if the data is staged as a separate commit and cherry-pick it into the table making it available to customers
* If not present in either of the above cases load the data and write it using the WAP functionality: stage the data with a specific id then cherry-pick it.
* At the end update the high-level metadata store.

## Iceberg’s version-hint.txt file improvement

An issue that Iceberg has with high throughput writes is properly resolving the latest table version to operate on. In some cases, the metafile (version-hint.txt) that provides Iceberg’s SDK the current version goes missing due to a known non-atomic rename/move operation.

When Iceberg commits a new version it will do so by using the HDFS CREATE (overwrite=true) API to replace the current content of version-hint.text with the new version value (an increment of the current one).

Internally AWS chose to implement this API as a DELETE + CREATE operation instead. This creates the possibility for a particular window of time, on each commit operation, where there is no version-hint.text file available.

This raises inconsistency since both the Iceberg reader and writer to depend on the version-hint file to resolve the version and select the appropriate metadata version files to load.

The solution relies on moving to a different implementation for persisting table version by leveraging the ADLS filesystem APIs guarantees — such as atomicity of CREATE(overwrite=false) and read-after-write consistency of list directory.

Hence, the implementation can be to _switch to preserving version using directory listing instead, so each writer will use CREATE(overwrite-=false) to create a new file to signal the new version while readers will have to list the versions directory and pick the highest value present at that particular moment in time_.

**Iceberg Community may address and fix it differently**.

## Further read

* [Scale-Part1](https://blog.developer.adobe.com/how-adobe-does-millions-of-records-per-second-using-apache-spark-optimizations-part-1-99f7e3432caa)
* [Scale-Part2](https://blog.developer.adobe.com/how-adobe-does-millions-of-records-per-second-using-apache-spark-optimizations-part-2-40074f8b557)
