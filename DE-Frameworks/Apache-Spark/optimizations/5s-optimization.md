# Introduction

1. Shuffle
2. Skew
3. Spill
4. Serialization
5. Storage

## Shuffle

Shuffle is a process in Apache Spark in which data is exchanged and repartitioned between worker nodes during computation. It is typically triggered when actions such as sorting, grouping, and joining are performed on the data.

What causes shuffle?

* Grouping and aggregation
* Joins
* Sorting
* Repartitioning

What are the issues related to shuffle?

* High network utilization. It can slow down the overall network speed of the cluster.
* Network bandwidth bottleneck
* Uneven data distribution. It can lead to the Skew problem.

What are the possible solutions?

* Minimizing data exchange over the network. When performing operations like joins or aggregations, it is important to select only the necessary rows and columns, rather than processing the entire dataset. This reduces the amount of data that needs to be shuffled across the network and can significantly improve performance.
* Broadcasting smaller tables. `spark.sql.autoBroadcastJoinThreshold` is a configuration used to control when a table should be broadcasted during a join operation. It has a default value of 10MB. However, you can to set the broadcast threshold to **1/10 of the available memory on worker nodes**. It is important to monitor the performance and adjust the threshold as needed to achieve the best performance.
* Tuning the number of partitions. As a general guideline, it is recommended to have 2–4 partitions per available CPU core in the cluster. You can use `repartition()` or `coalesce()`
* Use caching. `df.persist(StorageLevel.MEMORY_AND_DISK)` can be used to cache a data frame to memory, and if there is not enough available memory, Spark will cache the data to disk. `spark.storage.memoryFraction` parameter can be used to adjust the storage fraction. By default, Spark uses a storage fraction of 0.6, which means that 60% of the executor memory is allocated for caching and other storage needs. The remaining 40% of the memory is used for execution and other tasks.

## Skew

Data skew occurs when the data being processed is not evenly distributed across the worker nodes, which can lead to slower processing times and reduced efficiency. One of the common causes of data skew is an imbalance in the size of data between partitions, where one node processes significantly larger data than others.

What causes Skew to occur?

* **Imbalanced Data Distribution:** Uneven distribution of data across partitions due to the natural characteristics of the data can cause skew.
* **Data Quality Issues:** Skew can also occur due to data quality issues like a significant amount of missing or duplicate records, causing partitions of NULL or duplicated values to become much larger than partitions of non-null values.
* **Data Shift:** Changes in data distribution over time can result in some partitions becoming significantly larger than others.
* **Aggregation or Join:** When joining two large datasets or grouping a dataset on a skewed key, one partition may contain significantly more data than others, causing uneven processing times.

What are the issues with Skew?

* **Longer execution time:** Data skew in the workload can result in larger partitions taking longer to process, leading to longer overall execution times.
* **Resource wastage:** On the other hand, some executors may become idle while waiting for other executors to complete the processing of larger partitions. This can result in resource wastage, including unused executor memory and CPU cores.

Solutions

* **Salting technique:**

```python
from pyspark.sql import SparkSession
from pyspark.sql.functions import rand, round, concat, col, lit
import math
# Create a SparkSession
spark = SparkSession.builder.appName("data_skew").getOrCreate()

# turn off AQE
spark.conf.set("spark.sql.adaptive.enable", False)
spark.conf.set("spark.sql.adaptive.skewedJoin.enabled", False)

# Define the data as a list of tuples
fact_data = [("x", "value1"), ("x", "value2"), ("x", "value3"), ("y", "value4"), ("y", "value5"), ("z", "value6")]
dim_data = [("x", "value7"), ("y", "value8"), ("z", "value9")]

# Create the DataFrame from the list of tuples
df_fact = spark.createDataFrame(fact_data)
df_dim = spark.createDataFrame(dim_data)


# Determine number of partition
# Total size of DataFrame / size per partition (128MB is a good starting point)
# let's say our dataframe after join is 350MB
partitions = math.ceil(350 / 128) # 350 / 128 = 2.73, we round it up to 3

df_fact = df_fact.repartition(partitions, "_1")
df_dim = df_dim.repartition(partitions, "_1")

# Create salted dimension table
df_dim_salted = (df_dim
                .crossJoin(spark.range(partitions).toDF("salt"))
                .withColumn("salted_col1", concat(col("_1"),lit("_"),col("salt")))
                .drop("salt"))

# Create salted fact table
df_fact_salted = (df_fact
              .withColumn("salt", (rand() * partitions).cast("int"))
              .withColumn("salted_col1", concat(col("_1"), lit("_"), col("salt")))
              .drop("salt"))

# join two dataframes on sorted keys
df_joined = ( df_fact_salted
         .join( df_dim_salted, on="salted_col1", how="left"))
```

* **Adaptive Query Engine (AQE):** Post spark 3.2, this is handled automatically by AEQ.

```shell
spark.sql.adaptive.enabled=true
spark.sql.adaptive.skewJoin.enabled
spark.sql.adaptive.skewJoin.skewedPartitionFactor=7
spark.sql.adaptive.skewJoin.skewedPartitionThresholdInBytes=128MB
# skewedPartitionFactor depending on number of partitions that are optimal for your dataset 
# Below formula is a good starting point 
# skewedPartitionFactor = Total Size of Data / Size of a Partition (e.g. 128MB)
```

* **In-memory partitioning**: Use `repartition()` or `coalesce()`
* **Bucketing**: Keep similar data in same files within a folder (partition)

## Spill

A data spill occurs in Spark when there is insufficient space available to store data in an executor’s memory, and Spark writes the excess data from memory to disk. Later, when Spark needs to access the spilled data, it reads it back from the disk. This can result in a decrease in performance.

What causes Spills to occur?

* **Insufficient memory allocation:** When there is not enough memory allocated to executors to store the data in memory, Spark will spill the excess data to disk.
* **Processing large datasets:** When the size of the data being processed by Spark is too large to fit into the executer's memory, it can cause a data spill.
* **Skewed data:** When the data being processed by Spark is not evenly distributed across partitions, it can cause some partitions to receive more data than others, leading to data spill on the executor.
* **Complex transformations:** When the Spark application is performing complex transformations that require a large amount of memory, it can cause a data spill to the executor.
* **Large broadcast variables:** When the Spark application uses large broadcast variables, it can cause data spill on the driver nodes.
* **Collecting large data:** When the driver program collects data from worker nodes during actions such as `collect()`, `take()`, `first()`, or `count()`.

What are the issues with Spill?

* **Additional disk I/O overhead:** When data spills from memory to disk and is read back later, it introduces additional disk I/O overhead, which can significantly slow down the processing speed of Spark applications.
* **Out-of-memory errors(Executor):** If the spilled data is too large and there is not enough disk space available to store it, Spark may throw an out-of-memory error, which can cause the application to fail.
* **Out-of-memory errors(Drive):** For the driver, out-of-memory issues can occur when the application uses large broadcast variables or when large amounts of data are collected by the driver.

Solutions to Data Spill

* **Increase Memory Allocation to Drivers and Executors:** Allocate more memory to Spark jobs - `spark.driver.memory` & `spark.executor.memory`
* **Partitioning and Bucketing:** To optimize memory usage across executors and avoid excessive data processing, techniques such as partitioning and bucketing can be used to reduce the amount of data processed by each task. Optimization via partitioning and bucketing is already discussed in my previous article, you can refer to it here.
* **Adaptive Query Engine (AQE) — Predicate Push-Down and Dynamic Pruning Optimization:**
  * Predicate Push-down (row-wise optimization): This technique involves pushing query predicates as far down the execution plan as possible, which helps to reduce the amount of data that needs to be processed by filtering out unnecessary rows early in the query pipeline before any heavy operations like aggregate and join.
  * Dynamic Pruning Optimization (partition-wise and column-wise optimization): This technique involves dynamically pruning partitions and columns during query execution based on the data distribution and the query plan. By pruning unnecessary partitions and columns, AQE can further reduce the amount of data that needs to be processed, leading to faster query performance and more efficient memory usage.
* **Broadcast Join Tuning:** When performing join operations in Spark, the data used in the join may be broadcasted to all worker nodes in the cluster if the data is sufficiently small and below a predefined threshold controlled by the spark.sql.autoBroadcastJoinThreshold parameter, or if we explicitly perform broadcast(df) in the jobs.

To avoid data spill issues, it’s important to only broadcast data that is small enough to fit into the executor’s memory without affecting performance.

By default, the spark.sql.autoBroadcastJoinThreshold parameter works well for most use cases, so it's best to avoid changing it. However, if the data used in join operations are consistently larger than the default threshold, increasing it can improve query performance. Setting the threshold too high can lead to data spill issues while setting it too low can result in unnecessary data shuffling and slower query performance.
* **Reduce Data Collection to Driver:** When running Spark jobs, it’s important to minimize the amount of data collected by the driver to avoid performance issues and potential out-of-memory errors. In Spark, the driver is responsible for coordinating the execution of the job and collecting the results. If too much data is collected by the driver, it can overwhelm the driver’s memory and cause the job to fail.

One way to minimize the amount of data collected by the driver is to select only the necessary data before collecting it. In other words, rather than collecting an entire dataset or data frame, you can use filters, projections, and aggregations to select only the relevant data for the task at hand. This can help reduce the amount of data that needs to be transferred to the driver and improve the overall performance of the job.

If the data is too large to be collected by the driver, alternative methods such as writing the data to disk are recommended.

## Serialization

## Storage


https://medium.com/@chenglong.w1/the-5s-optimization-framework-for-spark-shuffle-optimization-f2b905e09bb0