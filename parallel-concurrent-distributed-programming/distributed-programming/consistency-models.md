# Consistency Models

## Strong Consistency
Most relational databases cannot tolerate weak consistency, say banking model. It is achieved by Serializability - when T1 and T2 are happening concurrently, all the transaction T1 is completed before transaction T2 initiates.

## Sequential Consistency
Parallel Programming obey sequential consistency. The behavior of this consistency model should be such that:
* Read(x) should be at least able to refer the data post write(x) with the same thread. Reads are locally consistent.
* write(x) across threads needs to be sequenced because thread T1 does not really know what is happening in thread T2. Write need to be synchronized and should be globally consistent.

## Temporal consistency

Time based consistency. When messages are passed, network delay can influence how each node receives the message. In temporal consistency, it is required that no node receives a past (historical) message later a last message. Example,

Say, node 1 sends a message M1 to node 2 and node 3. node 2 received M1 and responds back to it using M2 which is not sent to node 2 and node 3. If node 3 received M2 before M1 then this system becomes temporal inconsistent.

## Eventual Consistency
For a temporary period of time, the database may be in a inconsistent state, but it would become consistent eventually. It is important that we must guarantee to be consistent in a reasonable period of time. Most of the distributed system are designed for eventual consistency - this increases the availability condition of the distributed system. This is what is BASE (Basically Availability Soft state and Eventually Consistent) system.