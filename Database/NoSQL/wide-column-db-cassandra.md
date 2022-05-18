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
Replication factor 3 is ideal. We should have minimally 3 nodes.

Anti-clock wise and place the replicas in next two nodes.

The behavior is same for insert / update / etc. While fetch the data, we again pass the key which also goes through the same hashing. This again allows me to get to the node where my data is presents.

Now we can three situations -
1. All the nodes are up and live
2. A part of 3 nodes are up and live.
3. None of the nodes are up and live - Neither can we accept data nor can we serve it (key context). This is partial outage.

Replication Factor is a guideline, and not a rule.

Keep the database across multiple racks within Data Centre

### Multi-DC replication
Multi-DC Replication (multi-regions). Keyspace. How many copies you should keep in DC1 and how many in DC2. We can now specify keyspace (say, stockdb) which replication factor of 3 in DC1 and 2 in DC2. replication factor per DC.

HIPPA - keep the data in US ONLY. In such cases, say DC3 is in Europe. We specify replication factor of 0 for DC3 for that keyspace.

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

Primary KEY = ((Partition / Row key column(s)) + Clustering column(s))

Partition / Row key column(s) are Mandatory while clustering columns are optional.

Primary Key must be unique.

RK is hashed - this is what is hashed and determines which nodes the data would go.

If another record with same PK arrives, the previous data would be overwritten.

Volume, Velocity and Variety. Lets focus on velocity.


### Failure / Partition Tolerance by replication


```java
private static final String ellEventCql = "insert into all_evenets(event_type, data, created_hh, created_min, created_sec, created_nn, data) values (?, ?, ?, ?, ?, ?, ?)";

Session session = CassandraDAO.getEventSession();
BoundStatement boundStatement = getWriteableStatement(session, allEventCql);
session.execute(boundStatement.bind(....));
```

Every node can be a coordinator in a masterless architecture. Coordinator (whoever it is, at that point in time) handles all the incoming request (read/write)

> Please note that the data (each column) is timestamped using coordinator node's time.

There are multiple failure scenario when the coordinator is trying to write to a node and would face timeout. These failures are handled by cassandra using ***hinted handoff**.


### Consistency Level
Client thread (handling the operation) is waiting on coordinator to complete the request. Coordinator thread in turn is waiting on nodes (depending on replication factor) to complete the request. This is where consistency level comes into play.

Consistency Level defines at what point in time the coordinator can respond back to the client on completion of operation. This is tunable at the runtime. The valid values are:
* ONE (default) - the coordinator waits for one response from any of the node.
* QUORUM (strict majority w.r.t RF) - Majority responds. You can read about [QUORUM](read-write-quoroms.md)
* ALL - the coordinator waits for one response from all the node.
* EACH -
  Similarly, there are about 9 such values.

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

### Quorum

```
quorum = RoundDown(sum_of_replication_factors/2) + 1, where

sum_of_replication_factors = datacenter1_RF + datacenter2_RF + ... + datacentern_RF
```

> Quorum can help manage tolerance of replica down upto (Total number of node - quorum). So, using replication factor of 6, a quorum is 4 - the cluster can tolerate 2 replicas down.

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

