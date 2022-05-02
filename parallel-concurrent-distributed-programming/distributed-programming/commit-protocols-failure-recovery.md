# Commit Protocols and Failure Recovery

## Distributed Commit Protocols
It is primarily an agreement protocol. If there are n nodes, all the n nodes should agree to do things together (commit or abort in this context). Even if one node does not agree to align, the entire process can stall. It is a binary agreement protocol. Especially when we hit failure, it becomes even more important if we can reach to a consensus in finite time.