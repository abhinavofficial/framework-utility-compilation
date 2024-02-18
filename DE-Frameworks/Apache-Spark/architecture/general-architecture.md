# General Architecture

* [SparkContext](sparkContext.md)
* [RDD](RDD.md)

In your program, you can something called [SparkContext](sparkContext.md). If you r

Spark client (app master) has 4 major component:

1. RDD Graph
2. Scheduler
3. Block Tracker
4. Shuffle Tracker

Spark Worker

1. Task threads
2. Block Manaher

Spark Worker interacts with HDFS, HBase

## Cluster

Spark demands a set of machine. Cluster Manager, say YARN for machines in Hadoop Cluster, manages all the resources available within a set of machines. It is the cluster manager, which gives the set of machines an identity of being in cluster. Spark interacts with the cluster manager. Other examples of cluster manager is Apache Mesos, Kubernetes.

Standalone mode is when there is no external cluster manager and cluster management is provided by spark itself.

For the rest of this let's assume Spark is running with YARN and Hadoop. Spark's behavior would remain same irrespective of any cluster manager that we have chosen.

## Running the program

Spark program gets submitted to YARN. There are two important components within Spark -

1. Driver - It contains all your configuration settings. File format, compression, how many resources the program needs, etc. This starts first on one of the machine that YARN provides. Now this driver negotiates with YARN for resources to run the program. Driver asks for resources in the form of JVM. Inside this driver, the program would be asking for executors. If resources are available within the cluster, YARN will allocate them for your program.
2. Executor - It is just a combination of CPU and RAM on which program can execute. These are technically containers or JVMs. The programmer can decide how many executor is needed to run the program. You can ask for 2 executors with 4CPU and 8 GB RAM - this number can be approximately calculated for each program requirement.

The executors can be on the same machine - it is sole discretion of YARN. Once these executors are available, the driver starts pushing the programs on these executors. Driver then monitors all the executions. Eventually all the processing happens in parallel. Once both the executors complete their jobs, the output is streamed back to the driver. The driver then decides what to do (action). The executors are then deleted and eventually the driver at the end of program.

## Deciding on the number of executors

Golden rule: The more executors you have, more parallelization can happen and faster will be my execution.

### Dynamic resource allocation

You can specify to Spark to start with some number (min) of executors and go up to some number (max). Spark will monitor size of data and intensity of execution and ask for resource per its requirement.

## Handling the data

When you want to load the data into spark cluster, you can load them in partitions. Example, say you have 8GB file of sales.csv sitting in S3. I can request via program - give me this 8GB file in 8 partitions. This means, sparks reads the 8GB file and divide the data in 8 partitions.

Hash partitioner in spark takes care of this split.

These partitions are loaded into executors. You should have "ideally" same number of cores - sometimes not possible, and we should not be striving for it.

Having too many executors - It may lead to more network shuffles (or wide transformation) since one executor may not all the data required to complete the progress.

DAG or directed acyclic graph optimizes workflows. Lazy evaluation system.

Spark aims for binary API compatibility between releases, using MiMa, so if you are using the stable API theoretically, you generally should not need to recompile to run a job against a new version of Spark unless the major version has changed. In practice, we recommend recompiling all Spark jobs against the latest MINOR version as mistakes in binary compatibility have been known to happen.
MiMa is the Migration Manager for Scala and tries to catch binary incompatibilities between releases.
