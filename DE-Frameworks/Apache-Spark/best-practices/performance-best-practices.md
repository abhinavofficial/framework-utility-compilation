# Best Practices and Performance

## Best Practices

### Level of Parallelism

Clusters will not be fully utilized unless you set the level of parallelism for each operation high enough. Spark automatically sets the number of “map” tasks to run on each file according to its size (though you can control it through optional parameters to SparkContext.textFile, etc.), and for distributed “reduce” operations, such as groupByKey and reduceByKey, it uses the largest parent RDD’s number of partitions. You can pass the level of parallelism as a second argument (see the spark.PairRDDFunctions documentation), or set the config property spark.default.parallelism to change the default. In general, we recommend 2-3 tasks per CPU core in your cluster.

More on this [here](https://medium.com/@miquelangelandreu/use-the-right-level-of-parallelism-on-spark-fa58b20afc1c)

### Memory Usage of Reduce Tasks

Sometimes, you will get an OutOfMemoryError not because your RDDs don’t fit in memory, but because the working set of one of your tasks, such as one of the reduce tasks in groupByKey, was too large. Spark’s shuffle operations (sortByKey, groupByKey, reduceByKey, join, etc) build a hash table within each task to perform the grouping, which can often be large. The simplest fix here is to increase the level of parallelism, so that each task’s input set is smaller. Spark can efficiently support tasks as short as 200 ms, because it reuses one executor JVM across many tasks and it has a low task launching cost, so you can safely increase the level of parallelism to more than the number of cores in your clusters.

### ReduceByKey or groupByKey

Both groupByKey and reduceByKey produce the same answer, but the concept to produce results is different. reduceByKey is most suitable for large datasets because, in Spark, it combines output with a shared key for each partition before shuffling of data. While on the other side, groupByKey shuffles all the key-value pairs. GroupByKey causes unnecessary shuffles and transfer of data over the network.

### Maintain the required size of the shuffle block

By default, the Spark shuffle block cannot exceed 2GB. The better use is to increase partitions and reduce its capacity to ~128MB per partition that will reduce the shuffle block size. We can use repartition or coalesce in regular applications. Large partitions make the process slow due to a limit of 2GB, and few partitions don't allow to scale the job and achieve parallelism.

### Parallel Listing on Input Paths

Sometimes you may also need to increase directory listing parallelism when job input has large number of directories, otherwise the process could take a very long time, especially when against object store like S3. If your job works on RDD with Hadoop input formats (e.g., via SparkContext.sequenceFile), the parallelism is controlled via ``` spark.hadoop.mapreduce.input.fileinputformat.list-status.num-threads ```(currently default is 1).

For Spark SQL with file-based data sources, you can tune ```spark.sql.sources.parallelPartitionDiscovery.threshold``` and s```park.sql.sources.parallelPartitionDiscovery.parallelism``` to improve listing parallelism. Please refer to Spark SQL performance tuning guide for more details.

### File Formats and Delimiters

Choosing the right File formats for each data-related specification is a headache. One must choose wisely the data format for Ingestion types, Intermediate type, and Final output type. We can also Classify the data file formats for each type in several ways, such as we can use the AVRO file format for storing Media data as  Avro is best optimized for binary data than Parquet. Parquet can be used for storing metadata information as it is highly compressed.

### Small Data Files

Broadcasting is a technique to load small data files or datasets into Blocks of memory so that they can be joined with more massive data sets with less overhead of shuffling data. For instance, we can store small data files into n number of Blocks, and large data files can be joined to these data Blocks in the future as Large data files can be distributed among these blocks in a parallel fashion.

### No Monitoring of Job Stages

DAG is a data structure used in Spark that describes various stages of tasks in Graph format. Most of the developers write and execute the code, but monitoring of Job tasks is essential. This monitoring is best achieved by managing DAG and reducing the stages. The job with 20 steps is prolonged as compared to a job with 3-4 Stages.

### ByKey, repartition or any other operations which trigger shuffles

Most of the time, we need to avoid shuffles as much as we can as data shuffles across many, and sometimes it becomes very complex to obtain Scalability out of those shuffles. GroupByKey can be a valuable asset, but its need must be described first.

## Optimization Techniques

### Using Accumulators

Accumulators are global variables to the executors that can only be added through an associative and commutative operation. It can, therefore, be efficient in parallel. Accumulators can be used to implement counters (same as in  Map Reduce ) or another task such as tracking API calls. By default, Spark supports numeric accumulators, but programmers have the advantage of adding support for new types. Spark ensures that each task's update will only be applied once to the accumulator variables. During transformations, users should have an awareness of each task's update as these can be applied more than once if job stages are re-executed.

### Hive Bucketing Performance

Bucketing results with a fixed number of files as we specify the number of buckets with a bucket. Hive took the field, calculate the hash and assign a record to that particular bucket. Bucketing is more stable when the field has high cardinality, Large Data Processing, and records are evenly distributed among all buckets whereas partitioning works when the cardinality of the partitioning field is low. Bucketing reduces the overhead of sorting files. For Instance, if we are joining two tables that have an equal number of buckets in it, spark joins the data directly as keys already sorted buckets. The number of bucket files can be calculated as several partitions into several buckets.

### Predicate Pushdown Optimization

Predicate pushdown is a technique to process only the required data. Predicates can be applied to SparkSQL by defining filters in where conditions. By using explain command to query we can check the query processing stages. If the query plan contains PushedFilter than the query is optimized to select only required data as every predicate returns either True or False. If there is no PushedFilter found in query plan than better is to cast the where condition. Predicate Pushdowns limits the number of files and partitions that SparkSQL reads while querying, thus reducing disk I/O starts In-Memory Analytics. Querying on data in buckets with predicate pushdowns produce results faster with less shuffle.

### Zero Data Serialization / Deserialization using Apache Arrow

Apache Arrow is used as an In-Memory run-time format for analytical query engines. It provides data serialization/deserialization zero shuffles through shared memory. Arrow flight sends the large datasets over the network. Additionally, it has its arrow file format that allows zero-copy random access to data on-disks. It has a standard data access layer for all spark applications. It reduces the overhead for SerDe operations for shuffling data as it has a common place where all data is residing and in arrow specific format. 

### Garbage Collection Tuning using G1GC Collection

When tuning garbage collectors, we first recommend using G1 GC to run Spark applications. The G1 garbage collector entirely handles growing heaps that are commonly seen with Spark. With G1, fewer options will be needed to provide both higher throughput and lower latency. To control unpredictable characteristics and behaviours of various applications GC tuning needs to be mastered according to generated logs. Before this, other optimization techniques like **Streaming and Real-Time Analytics Solutions**, in the program’s logic and code must be applied. Most of the time, G1GC helps to optimize the pause time between processes that are quite often in Spark applications, thus decreases the Job execution time with a more reliable system. More details [here](garbage-collection.md)

### Memory Management and Tuning

As we know that, for computations such as shuffling, sorting and so on, Execution memory is used whereas for caching purposes storage memory is used that also propagates internal data. There might be some cases where jobs are not using any cache; therefore, cases out of space error during execution. Cached jobs always apply less storage space where the data is not allowed to be evicted by any execution requirement. In addition, **Real-Time Streaming Application with Apache Spark** can be done. We can set spark.memory.fraction to determine how much JVM heap space is used for Spark execution memory. Commonly, 60% is the default. Executor memory must be kept as less as possible because it may lead to delay of JVM Garbage collection. This fact is also applicable for small executors as multiple tasks may run on a single JVM instance. More details [here](memory-management.md)

### Data Locality

In it the processing tasks are optimized by placing the execution code close to the processed data, called data locality. Sometimes processing task has to wait before getting data because data is not available. However, when the time of spark.locality.wait expires, Spark tries less local level, i.e., Local to the node to rack to any. Transferring data between disks is very costly, so most of the operations must be performed at the place where data resides. It helps to load only small but required the amount of data along with **test-driven development for Apache Spark**.

* _PROCESS_LOCAL_ data is in the same JVM as the running code. This is the best locality possible.
* _NODE_LOCAL_ data is on the same node. Examples might be in HDFS on the same node, or in another executor on the same node. This is a little slower than PROCESS_LOCAL because the data has to travel between processes.
* _NO_PREF_ data is accessed equally quickly from anywhere and has no locality preference
* _RACK_LOCAL_ data is on the same rack of servers. Data is on a different server on the same rack so needs to be sent over the network, typically through a single switch.
* _ANY_ data is elsewhere on the network and not in the same rack.

Spark prefers to schedule all tasks at the best locality level, but this is not always possible. In situations where there is no unprocessed data on any idle executor, Spark switches to lower locality levels. There are two options: a) wait until a busy CPU frees up to start a task on data on the same server, or b) immediately start a new task in a farther away place that requires moving data there.

| Property Name               | Default           | Meaning                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Since Version |
|-----------------------------|---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------|
| spark.locality.wait         | 3s                | How long to wait to launch a data-local task before giving up and launching it on a less-local node. The same wait will be used to step through multiple locality levels (process-local, node-local, rack-local and then any). It is also possible to customize the waiting time for each level by setting spark.locality.wait.node, etc. You should increase this setting if your tasks are long and see poor locality, but the default usually works well. | 0.5.0         |
| spark.locality.wait.node    | spark.locality.wait | Customize the locality wait for node locality. For example, you can set this to 0 to skip node locality and search immediately for rack locality (if your cluster has rack information).                                                                                                                                                                                                                                                                     | 0.8.0          |
| spark.locality.wait.process | spark.locality.wait | Customize the locality wait for process locality. This affects tasks that attempt to access cached data in a particular executor process.                                                                                                                                                                                                                                                                                                                    | 0.8.0          |
| spark.locality.wait.rack    | spark.locality.wait | Customize the locality wait for rack locality.                                                                                                                                                                                                                                                                                                                                                                                                               | 0.8.0          |

### Using Collocated Joins

Collocated joins make decisions of redistribution and broadcasting. We can define small datasets to be located into multiple blocks of memory for achieving better use of Broadcasting. While applying joins on two datasets, spark First sort the data of both datasets by key and them merge. But, we can also apply sort partition key before joining them or while creating those data frames INApache Arrow Architecture. This will optimize the run-time of the query as there would be no unnecessary function calls to sort.

### Caching in Spark

Caching in **Apache Spark with GPU** is the best technique for its Optimization when we need some data again and again. But it is always not acceptable to cache data. We have to use cache () RDD and DataFrames in the following cases -

* When there is an iterative loop such as in Machine learning algorithms.
* RDD is accessed multiple times in a single job or task.
* Or, the cost to generate the RDD partitions again is higher.

Cache () and persist (StorageLevel.MEMORY_ONLY) can be used in place of each other. Every RDD partition which gets evicted out of the memory is required to be build again from the source that still is very expensive. One of the best solutions is to use persist (Storage level.MEMORY_AND_DISK_ONLY ) that would spill the partitions of RDD to the Worker's local disk. This case only requires getting data from the Worker's local drive which is relatively fast.

### Executor Size

When we run executors with high memory, it often results in excessive delays in garbage collection. We need to keep the cores count per executor below five tasks per executor. Too small executors didn’t come out be handy in terms of running multiple jobs on single JVM. For Instance, broadcast variables must be replicated for each executor exactly once, that will result in more copies of the data.

### Spark Windowing Function

A window function defines a frame through which we can calculate input rows of a table. On individual row level. Each row can have a clear framework. Windowing allows us to define a window for data in the data frame. We can compare multiple rows in the same data frame. We can set the window time to a particular interval that will solve the issue of data dependency with previous data. Shuffling in Apache Beam is less on previously processed data as we are retaining that data for window interval.

### Watermarks Techniques

Watermarking is a useful technique in its Optimization that constrains the system by design and helps to prevent it from exploding during the run. Watermark takes two arguments -

* Column for event time and
* A threshold time that specify for how long we are required to process late data

The query in Apache Arrow Architecture will automatically get updated if data fall within that stipulated threshold; otherwise, no processing is triggered for that delayed data. One must remember that we can use Complete-mode side by side with watermarking because full mode first persists all the data to the resulting table.

### Data Serialization

Apache Spark optimization works on data that we need to process for some use cases such as Analytics or just for movement of data. This movement of data or Analytics can be well performed if data is in some better-serialized format. Apache Spark supports Data serialization to manage the data formats needed at Source or Destination effectively. By Default, it uses Java Serialization but also supports Kryo Serialization. By default, Spark uses Java’s ObjectOutputStream to serialize the data. The implementation can be through the java.io.Serializable class. It encodes the objects into a stream of bytes. It provides lightweight persistence and flexible. But it becomes slow as it leads to huge serialized formats for each class it is used in. Spark supports Kryo Serialization library (v4) for Serialization of objects nearly 10x faster than Java Serialization as it is more compact than Java.

### Broadcasting Large Variables

Using the broadcast functionality available in SparkContext can greatly reduce the size of each serialized task, and the cost of launching a job over a cluster. If your tasks use any large object from the driver program inside of them (e.g. a static lookup table), consider turning it into a broadcast variable. Spark prints the serialized size of each task on the master, so you can look at that to decide whether your tasks are too large; in general tasks larger than about 20 KiB are probably worth optimizing.
