# Consistency Models

## Strong Consistency
Most relational databases cannot tolerate weak consistency, say banking model. It is achieved by Serializability - all the transaction T1 is completed before transaction T2 initiates.

## Sequential Consistency
Parallel Programming obey sequential consistency. The behavior of this consistency model should be such that:
* Read(x) should be at least able to refer the data post write(x) with the same thread. Reads are locally consistent.
* write(x) across threads needs to be sequenced because thread T1 does not really know what is happening in thread T2. Write need to be synchronized and should be globally consistent.

## Temporal consistency

Time based consistency. When messages are passed, network delay can influence how each node receives the message. In temporal consistency, it is required that no node receives a past (historical) message later a last message.

## Eventual Consistency
For 

## Case Study: Cassandra Masterclass Architecture

Cassandra Ring. Round robin. 


### Partitioner
Hashmap
put(k,v)
get(k)

Partition - Murmur3. It is a byte array and it producer long (-2^63 to 2^63-1)
Algo should be fast, replicable
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