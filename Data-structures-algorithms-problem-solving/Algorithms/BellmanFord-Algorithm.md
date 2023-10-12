# Bellman Ford Algorithm

Bellman Ford Algorithm (BFA) is also an algorithm to find the shortest distance from the source to all vertices. It overcomes the limitation with DJK of negative weight(s) to the edge. It follows dynamic programming for its algorithm, and not greedy as was used by Dijkstra's algo. Time complexity of Bellman Ford is higher than DJK's.

Algorithm

Perform the below operation V-1 time, where V is the number of vertices.

```text
for all edges[u, v]
    if dist[u] + wt(u, v) < dist[v]
        dist[v] = dist[u] + wt(u, v)
```

This works with all kind of graphs - cyclic/acyclic directed/undirected graphs except Negative weight cycles.

Negative weight cycle: If wt(a->b) + wt(b->c) + wt(c->a) < 0, it is called negative weight cycle. There is no good sense to calculate the shortest distance in such scenario. One quick way to test this condition is to run BFA one more time and the distances are still being updated - this is a litmus test.

Time complexity: O(V*E)

Implementation is present at [here](https://github.com/abhinavofficial/prep-inteview/blob/main/Graph.py)
