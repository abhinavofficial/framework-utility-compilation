# How to manage data growth with Iceberg?

## Four techniques

Percentage data growth can sometimes beat the revenue growth. This is an indicator to evaluate if something is not right.

* Storage Optimization
* Aggressive janitors
* Incremental processing
* Workload management

## Storage Optimization

> Key principle - Same data but less space.

Use of 4 C's - Columnar, Compaction, Compression and Clustering

|4Cs        |What does it mean                                 |Comment                               |
|-----------|--------------------------------------------------|--------------------------------------|
|Columnar   | Use of parquet or orc                            |                                      |
|Compaction | Combining small files                            | Use binpack or sort                  |
|Compression|zstd, gzip, snappy                                | Trade off between compute and storage|
|Clustering | sorting and co-locating records in an optimal way|                                      |

Declarative

```text
write.format.default
write.target-file-size-byte
write.parquet.compression-codec
partition-spec
sort-order
```

This can be deferred for streaming application.

## Aggressive janitors

> Key principle - Only pay for what you use

* Unused Tables
* Unused data within tables
* Historical data and orphaned files

### Unused Tables

* Usage and Access reporting should be implemented.
* Lineage is huge plus.
* Consider user-level scratch spaces with more aggressive TTLs unless tagged to be retained. Do a soft delete where table can be restored or just remove user access.

You can use tagged based S3 lifecycle management.

### Unused data within tables

* Time-series data has steep value decay curve.
  * Set record-level TTL to delete old data or systematically move to cheaper storage tier.
* Beware of large, unstructured text columns
  * Column level usage is helpful

### Historical data & orphaned files

* Expired -> Clean up
* Historical
* Orphan files (failed write job) - can use Spark procedure

## Incremental processing

> The only constant is change

* Iceberg snapshots provide a lineage change history.
* Expressive SQL
  * MERGE, UPDATE, and DELETE
  * Let engine optimize plans
    * Dynamic partition pruning
    * Storage-partitioned joins

## Workload management

> Easily shift workloads based on data volume, latency, and performance needs.

There are many engines which supports Iceberg. Spark, Flink, DuckDB, Starrocks, Impala, Presto, Trino, Hive, ClickHouse, Druid, Doris, Python.

## Implementation in S3
