# Kosaraju's Algorithm

Kosaraju's Algorithm is used to find strongly connected components within a graph. Strongly connected component is a subgraph where we can reach to every vertex from every other vertex.

This algo uses DFS (of course, slightly modified), rather reverse DFS. It also implicitly assumes understanding of vertices topology (node sorted such that dependencies come before the vertex).

Algorithm

* Get nodes in stack (topological sort) - Time Complexity: O(V + E)
* Transpose the graph - Time Complexity: O(V + E)
* Do DFS according to stack nodes on the transpose graph. - Time Complexity: O(V + E)

Overall Time Complexity: O(V + E)

Implementation is present at [here](https://github.com/abhinavofficial/prep-inteview/blob/main/Graph.py)
