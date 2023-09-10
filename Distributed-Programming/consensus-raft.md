# The Raft Consensus Algorithm

Raft is a distributed consensus algorithm designed to solve the problem of getting multiple servers to agree on a shared state, even during failures.

It selects one of the participating node as leader and others to be followers. The client submits the request to the leader node and followers execute per leader's command.

The only problem is when the leader fails. In this case, a leader selection protocol is generated and one of the nodes can accept to become the leader. One of the conditions is that follower, who becomes the leader has the best state of information, so that there is consistency. This is also called view change protocol.

> Source: https://raft.github.io

## Building a large-scale distributed storage system based on Raft

> Source: https://cncf.io/blog/2019/11/04/building-a-large-scale-distributed-storage-system-based-on-raft/

[Raft Consensus Algorithm](https://raft.github.io/)
