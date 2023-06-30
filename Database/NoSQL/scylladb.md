# ScyllaDB
 Built on top of Cassandra for performance and scalability.
 
## Terminology

**Cluster**: Remember that a cluster is a collection of one or more ScyllaDB nodes, which essentially work together in a distributed fashion to process your workloads. For production workloads, we recommend that your topology is composed of at least 3 nodes for high availability.

**Keyspace**: Within a Cluster, we create Keyspaces. Keyspaces are a top-level container which aggregates tables. Within a keyspace, you can define to which regions and how many replicas – or copies of your data – exist. For example, you can have a keyspace which replicates data from your table to nodes physically located within the US and Canada, and another keyspace which replicates your data only to nodes placed within the Europe Union, for compliance purposes.

**Tables**: Inside a keyspace, you create Tables. Which are essentially the actual data structure that is going to be holding your data, and where you will be doing your data modeling.

**Partition Key**: When you create a table, you need to define what is going to be its Partition Key. A partition key is the column that you chose during your initial data modeling that will tell your ScyllaDB cluster how it should shard your data across the cluster. It is extremely important to choose a partition that has enough high cardinality and which allows your cluster to scale without introducing any bottlenecks or imbalances.

## Starting Up

## Playing with cqlsh

```shell
cslsh> CREATE KEYSPACE scylla_labs WITH replication = {‘class’: ‘NetworkTopologyStrategy’, ‘datacenter1’: 1};
```
WITH replication = 
* {‘class’: ‘NetworkTopologyStrategy’ – Here we specify our replication strategy. ScyllaDB supports multiple different types of replication, and we highly recommend you to stick with the NetworkTopologyStrategy for your production workloads. 

* ‘datacenter1’: 1 – We then specify that we want our data to be replicated to a single node within the datacenter1. By default, ScyllaDB uses datacenter1 when you do not make use of a Cloud snitch and do not explicitly give your node a data center name. The datacenter name is within the ```nodetool status``` output we have previously seen. Finally, we tell the database that it should only replicate data once to this datacenter, specifically because we are running a single-node cluster. 

> In a real production environment, you would normally want your data to be replicated to at least 3 nodes. Keep that in mind, as an incorrect replication factor can introduce data loss and a single point of failure in your deployment!

```shell
cslsh> CREATE TABLE scylla_labs.my_table (id int, time timestamp, data blob, PRIMARY KEY(id, time));
```

* The scylla_labs.my_table parameter takes the format of <keyspace_name>.<table_name>, which can be very useful to distinguish between one keyspace from another. Here, we are creating table my_table inside the keyspace scylla_labs. You may also work inside a keyspace exclusively, provided you use the command USE <keyspace_name>; beforehand.
* We specify our table columns after, within the “id int, time timestamp, data blob” statement. Here, we have defined three fields: id, time and data. Each one with its own data type. 
* Out of the column names we specified earlier, we need to choose one to be our partition key. That is, which column (or columns) will ScyllaDB use in order to shard our data automatically among the cluster. We have chosen the id column to be our partition key. But what about the time column? The time column is our clustering key. In particular, if you are familiar with DynamoDB, think of the partition key as your hash key and your clustering key as the sorting key as in Dynamo.

## Introduction to cassandra-stress

cassandra-stress is a benchmarking tool that ScyllaDB inherited from the Apache Cassandra project and extended it (with capabilities such as shard-awareness).

There are two primary reasons to use cassandra-stress when working with ScyllaDB:
* Running simple benchmarks against a ScyllaDB cluster. You have the flexibility to specify several configuration parameters, such as the throughput, number of operations to run, threads to execute, and duration of the test.
* You may stress the cluster by using your own defined profiles, which allows you to test whether your data modeling scales, as well as help you to simulate real world situations, such as node failures, as we’ve seen during the first part of this presentation.

Getting started with cassandra-stress is really simple. Inside our ScyllaDB container, the first thing that we’ll generally want to do is to ingest data to the cluster. It is important that we have data beforehand because, later on, if we want to run read tests or a mixed read/write workload, the data to read from will already exist in the database.

> In an actual production environment, you always want to run cassandra-stress on dedicated machines rather than from your ScyllaDB nodes. Doing so guarantees that your stressor clients (in this case, cassandra-stress) won’t steal valuable computing resources from your database.

Here’s an example of how to write to a ScyllaDB cluster:
```shell
$ cassandra-stress write n=1M -rate threads=8 -node 172.17.0.2

# The write command tells cassandra-stress that it should perform a write operation.

# The n=10K is an argument to the previous write parameter, and specifies that we want to run 10 thousand operations in total.

# The -rate threads=2 introduces our first cassandra-stress option: -rate , which allows us to control thread count, rate limiting, among other aspects. We specify that we want cassandra-stress to confine itself to only 2 threads.

# Finally, we specify the -node option, and tell cassandra-stress to connect to 172.17.0.2 IP address and use it as a contact point.

# If you are unsure what's the IP address of your ScyllaDB container, you may either check it out from the nodetool status
```

> You may run cassandra-stress help to learn more about multiple commands and options! For each command or option, you may also specify the help argument to receive additional information. For example: cassandra-stress help write will show up all information related to the write command.


