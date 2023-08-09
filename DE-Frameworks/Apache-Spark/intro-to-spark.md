# Intro to Spark

## Basic of spark
* 2009-2010: Spark was created as an execution engine in AmpLab, and nothing more and as a competitor of map reduce.
* It is really fast. In the benchmark test, Spark ran workloads 100x faster than map reduce.
* 2012-2013: Spark was open-sourced. Matei Zaharia, (Bill Chambers, Ali Hoz)
* 2015-2016: Become popular.
* It is very easy to program and supports Java, Scala, Python, R and SQL.
* We can run it anywhere. It is platform independent. Can run in cluster mode or standalone mode.
* It can read and write data from anywhere. Since it is really an execution engine, it does not have any storage. csv, json, txt, parquet, avro, xml
* Version - 1.x is now deprecate. Spark 2.x is very common, and we will use it. Spark 3.x is now coming up.

## Why to use Spark
* Lot of data and Big Data
* Of course, the above reasons in Basic of Spark
* Hadoop map-reduce was general batch processing (2004-2013). 
* During 2007-2015: Many specialized tools came into being to support various use cases.
  * Hive - SQL on Hadoop
  * Impala
  * Tez
  * Drill
  * S4
  * Dremel
  * Pregel
  * Mahout - ML framework on Hadoop
  * Storm - Real-time analytics
  * Flink - Streaming
  * Giraph and GraphLab - For graph based data
* 2014 onward - Spark general unified engine. (Spark Core - > Spark SQL, Spark Streaming, MLib, GraphX)
* It gets its speed via in-memory processing.
* Databricks as a company was created by the original authors of Spark and is the undisputed leader of spark. It is cloud-based.

## Summary
* Fast distributed computing using in-memory primitives
* Offers real-time stream processing, interactive processing, graph processing, as well as batch processing in distributed, parallel process environment.
* Handles limitations by previous paradigms like intensive disk usage, poor use of memory, linear workflows constraining use cases
* Complex data flow definitions can be specified using Directed Acyclic Graphs (DAGs)
  * Vertices are RDDs and edges are the operations applied on this RDDs.
  * Prepared DAG is submitted to DAG Scheduler
  * Processed in stages separated by data restructuring, and execution plan is optimized.
* Data locality - Processing is scheduled based on closeness to data at process, node, rack levels
  * Maps to distributed file systems like HDFS for optimal processing and on-disk persistence
* Automatic partitioning and node distribution
* Fault-tolerance by storing complete lineage and the ability to replay the whole pipeline for the failed partition

Source: [Wiki](https://en.wikipedia.org/wiki/Apache_Spark)
