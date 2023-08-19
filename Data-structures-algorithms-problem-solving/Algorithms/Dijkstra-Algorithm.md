# Dijkstra's Algorithm
This is used to find the shortest distance from the source to all vertices. This can be directed or undirected graph. In most cases, the edges are weighted. If weights are not defined, you can imagine this to be 1 for all practical purposes.

One of the key concept to understand here is **relaxation**. If the distance of source, S to destination, U is say, x and distance of source, S to another destination, V is say, y, and there is another direct path between U and V (aka weight of the edge (U to V)) is z, then shortest distance from S to v would be minimum of y and (x+z).
> Please note S -> U or S -> V need not be direct edge - it could have travelled via many other vertices.
```python
if dist[u] + wt < dist[v]:
    dist[v] = dist[u] + wt
```

Algorithm:
* It works on BFS
* It is based of greedy algorithm, which is really to choose the best option at that given point in time. This is almost like thinking in the minimum cost problem scenario as how to keep the cost as minimum at every step of calculation.
* We can use queue. We can use priority queue (implemented as MinHeap)
* Create an array or list, say distance_array, which captures the distance of each vertex (1 to n), from source, say 0 including 0. Of course, the first element value would 0 (distance of S from S).
* Initialize the distance_array to inf for all element, except element 0 which has value of 0.
* At every step - recursively 
  * Find the node for which visited is false and distance is shortest.
  * Mark this node as visited
  * Now try to update the distance of its neighbours (using relaxation)

Implementation Plan
* To find out which vertex has the shortest distance, we plan to have a class, pair which holds the node and distance value.
* The pair object is stored in MinHeap, so the shortest distance is at the top and is return in O(1).

Implementation is present at https://github.com/abhinavofficial/prep-inteview/blob/main/Graph.py

Time Complexity: O(E + E*logV)
E*logV is due to priority queue
