# Tarjan's Algorithm

This is used for finding bridges in a graph. However, it can be used to find articulation point, strongly connected component and topological sorting. To use Tarjan's algorithm, we use a concept called discovery time.

Tarjan's algorithm works on dfs. Discovery time is notional sense of tracking time when we reached at a particular vertex within the dfs.

While Tarjan's algorithm can be used for directed graphs, it finds special meaning with undirected graph.

The idea here is that there exists a bridge, which is an edge between vertex u and v, such that discovery time u is always smaller than discovery time of v. If this is not true, there must be another to reach to v from u. In general, any component connected to u must be discovered before any component connect to v if there is a bridge between them.

```text
dt[u] < lowest(dt[v])
```

## Algorithm

We use two data structures here.

* First is to track the discovery time, which is of size of number of vertices.
* Second is to store the lowest discovery time of all neighbors including itself against that array element index. The size of this structure is also the same as number of vertices.
* We additionally need to track parent for each vertex.
* Initialize discovery time / the lowest discovery time and parent with 1 and -1 respectively for vertex 0
* Now do the dfs and keep updating discovery time and lowest discovery time for each neighbor, not updating the parent.

```text
if e.dest == parent
    continue
```

* If neighbor is not visited, then

```text
if not visited[e.dest]:
    dfs with e.dest
    low[curr] = min(low[current], low[neighbor])
    if dt[curr] < low[neigh]
        print(curr + neigh]
```

1. Maintain the lowest discovery time

```text
if visited(e.dest):
    low_dt[current] = min (low_dt[current], dt[neigh]) # the logic will be become more clear soon.
```

Implementation is present at [here](https://github.com/abhinavofficial/prep-inteview/blob/main/Graph.py)
