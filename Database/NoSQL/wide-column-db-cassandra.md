# Apache Cassandra

## Architecture

https://docs.datastax.com/en/cassandra-oss/3.0/cassandra/architecture/archTOC.html

## Case Study: 

### Cassandra Masterclass Architecture

Cassandra operates using a set of nodes, often called Cassandra Ring. This is arranged in P2P fashion. Let's assume there are two clients, A and B (which can be different instances of the same application). Client can engage with any node for any specific operation in a round robin fashion. In fact, as a part of driver in Cassandra, we can specify DC aware round robin policy. It allows the system to distribute the load to multiple node. While interacting if a particular node goes down, client A can interact with another node. From CAP theorem perspective, the system remains available in such situation (we will discuss consistency later). This is what masterless architecture of Cassandra is.

Let's now understand how the data is saved in Cassandra - this would help us understand the consistency of the system.

### Partitioner
Cassandra using partitioner to distribute the data within its node. Partitioner in essence is a hashmap. 

Hashmap minimally has two methods: ```put(k,v)``` for inserting or upserting and ```get(k)``` for selection. Hashmap takes the key, runs it via a hashing algorithm which producing a hashvalue,  representative for a location (also called a bucket) and puts value at that bucket. In case of hash collision, we can have a list or tree - lets not worry about it to understand the concept.

The partitioner in Cassandra is called Murmur3 (created by Austin Appleby). Murmur3 takes a byte array, and it produces long (-2^63 to 2^63-1) which is also called token **consistently**. In general, any hashing algorithm should be fast and repeatable.

Idea is that based on this token ranges, we can put the data in different nodes. Now, let's bring in replication factor complexity in. Replication factor 3 is ideal which means we should have minimally 3 nodes in a distributed system get the benefits of replication factor. In a distributed system of n nodes therefore, each node would have equally distributed token ranges i.e ```token range / n``` in an ideal scenario. So, if the replication factor is 3, then the first node is determined by this token range and then places its replicas in next two nodes in clock wise. There are concepts of v-nodes but let's not get into that but, rather understand token based storage on a simplistic case as described.

The behavior is same for insert / update / etc. While fetch the data, we again pass the key which also goes through the same hashing. This again allows me to get to the node where my data is present. Even in a very large cluster, we really do not care but rather just three based on our token and replication factor. Now we can three situations -
1. All the nodes are up and live - No problem.
2. A part of 3 nodes are up and live - Insert or Upsert data on active node and worry about making the consistent on dead node (once up) 
3. None of the nodes are up and live - Neither can we accept data nor can we serve it (key context). This is partial outage. So, we should have the design so that not all the replicas for a particular key is down. One way to do this is - Keep the database across multiple racks within Data Centre. Even better is using Multi-DC replication but at a slight performance cost.

> Replication Factor is a guideline, and not a rule. See point 2 above, we did not really fail the system when RF was not met.

### Multi-DC replication
When we have multiple regions, and we want to distribute the data in multi-DC seemlessly (meaning Replicating data across multi-regions/ multi-dc), Cassandra uses a concept called Keyspace where we can specify how many copies you should keep in DC1 and how many in DC2. We can specify keyspace (say, stockdb) which replication factor of 3 in DC1 and 2 in DC2. Replication factor is defined per Data Centre.

Keyspace ~ Database or Schema - Just drawing an analogy. We can create tables in keyspace.

HIPPA - keep the patient's data in US ONLY. In such cases, say DC3 is in Europe. We specify replication factor of 0 for DC3 for that keyspace.

### Visualizing the Primary Key

```sql
CREATE TABLE meter_data (
meter_id text,
date     int, //format as yyyymmdd number (more efficient)
created_hh int,
created_min int,
created_sec int,
created_nn int,
data text,
PRIMARY KEY ((meter_id, date), created_hh, created_min)
);
```

Primary KEY = (**(Partition / Row key column(s)**) + _Clustering column(s)_)

**Partition / Row key column(s)**, referred here as RK are mandatory while clustering columns are optional.
Primary Key must be unique.
RK is hashed - this is what is hashed and determines which nodes the data would go.
If another record with same PK arrives, the previous data would be overwritten.

In Big Data, three Vs refers to Volume, Velocity and Variety. Let's focus on velocity in the above example. If you have a PK which includes meter_id, data, created_hh and create_min, you are limited the velocity to ingestion to 1 min. To increase it to ingesting every sec, we may need to add create_sec and even further, we can add create_nn for nanosecond velocity capture.

> To manage velocity, you must define your primary key carefully.

### Failure / Partition Tolerance by replication

Replication gives us higher data availability while system availability is achieved by masterless architecture.

Consider the code which manages interaction between a client and Cassandra:
```java
private static final String ellEventCql = "insert into all_evenets(event_type, data, created_hh, created_min, created_sec, created_nn, data) values (?, ?, ?, ?, ?, ?, ?)";

Session session = CassandraDAO.getEventSession();
BoundStatement boundStatement = getWriteableStatement(session, allEventCql);
session.execute(boundStatement.bind(....));
```

Whenever we say client interacts with Cassandra, we really mean Cassandra coordinator. The interesting feature of Cassandra is that every node can be a coordinator since it follows a masterless architecture. Coordinator (whoever it is, at that point in time) handles all the incoming request (read/write).

> Please note that the data (each column) is timestamped using coordinator node's time.

There are multiple failure scenario when the coordinator is trying to write to a node and would face timeout. These failures are handled by cassandra using ***hinted handoff** (the failure in which coordinator does not get a response back).

There are two other concepts to dealt with later along with hinted handoff - it is **immutability** and **idempotent operation**.


### Consistency Level
Client thread (handling the operation) is waiting on coordinator to complete the request. Coordinator thread in turn is waiting on nodes (depending on replication factor) to complete the request. This is where consistency level comes into play.

Consistency Level defines at what point in time the coordinator can respond back to the client on completion of operation. This is tunable at the runtime. The valid values are:
* ONE (default) - the coordinator waits for one response from any of the node.
* QUORUM (strict majority w.r.t RF) - Majority responds. You can read about [QUORUM](../../parallel-concurrent-distributed-programming/distributed-programming/read-write-quoroms.md)
* ALL - the coordinator waits for one response from all the node.
* EACH - Similarly, there are about 9 such values.

```java
protected static BoundStatement getWriteableStatement(Session session, 
        String cql, boolean setAnyConsistencyLevel) {
    PreparedStatement statement = session.prepare(cql); //ideally prepare a statement once per session 
    if(setAnyConsistencyLevel) {
        statement.setConsistencyLevel(ConsistencyLevel.QUORUM);
    }
    
    BoundStatement boundStatement = new BoundStatement(statement);
    return boundStatement;
}
```
This can be applied both to read and write.


> Quorum can help manage tolerance of replica down upto (Total number of node - quorum). So, using replication factor of 6, a quorum is 4 - the cluster can tolerate 2 replicas down.

>If your consistency level is ALL, then the request would fail even if there is one node down. 

Based on gossip (which provides an insight on load each node is handling currently), coordinator hands off the request to node(s) with minimum load (first). So, if consistency level is ONE, it would pick node with the least load.

> Please note gossip is used only for reads and not write. For write, nodes are accessed based on hash per PK and replication factor.

```91228 93949 - Sahil```

### Write consistency level

#### Write ONE
Send requests to all replicas in the cluster applicable to the PK
Wait for ONE ack before returning to client
Other acks later, asynchronously

#### Write QUORUM
Send requests to all replicas in the cluster applicable to the PK
Wait for QUORUM ack before returning to client
Other acks later, asynchronously

With performance reduced, we have increased the confidence of data availability.

### Read consistency level

#### READ ONE
Send requests to all replicas in the cluster applicable to the PK
Wait for ONE ack before returning to client
Other acks later, asynchronously

#### READ QUORUM
Read from one fastest/preferred node and request digest from other replicas to reach QUORUM and returns most up-to-data data to client by comparing timestamp.

#### READ ALL
Read from one fastest/preferred node and request digest from all other replicas and returns most up-to-data data to client by comparing timestamp.
This gives me the highest consistency. High availability is given by masterless architecture. But it does not give me partition tolerance - assume even if one node is down.

With performance reduced, we have increased the confidence of data availability.

**Consistency Level - Summary**

* ONE - Fast write, may not read the latest written value
* QUORUM / LOCAL_QUORUM - Strict majority w.r.t. Replication Factor, Good Balance
* ALL - Not the best choice. Slow, no high availability

> If (node_written + nodes_read) > replication factor, then you can get immediate consistency

> Replication Factor should be set to off number

You would keep hitting the question on whether you can live with eventual consistency in the wake of higher performance. It is therefore important to understand how eventual is eventual consistency? [Best Link](http://pbs.cs.berkeley.edu/#/demo) demonstrates the same.

## Hands on
sed -i 's=MODE_DATACENTER=dc1=g' cassandra.yaml
nodetool status

Cluster name should be exact
Self
seed_list should be first one or existing one.

both nodes should be able to talk to each other - so use the correct security group.

```shell
csqlsh `hostname -I` -u cassandra -p cassandra
#or
csqlsh `hostname -I` -u cassandra
```

Now within csqlsh
```
ALTER KEYSPACE 'system_auth' WITH REPLICATION = {'class': 'NetworkTopologyStrategy', 'dc1': 3 }

CREATE KEYSPACE IF NOT EXISTS stockdb WITH REPLICATION = {'class': 'NetworkTopologyStrategy', 'dc1': 3 };

create table stockdb.user {
  user_id VARCHAR,
  location VARCHAR,
  display_name VARCHAR,
  first_name VARCHAR,
  last_name VARCHAR,
  PRIMARY KEY (user_id, location)
};
```

> Consistency Level does not impact the number of nodes that we would reach out to while write. In reads, consistency level controls how many nodes we want to read from based on gossip.