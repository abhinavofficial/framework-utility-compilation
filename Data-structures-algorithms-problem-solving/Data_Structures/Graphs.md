# Graphs

Graph is a network of nodes.

## Key concepts

* Vertex: Node
* Edge: source, destination, direction (optional) and weight (optional)
* Weight
* Unweighted
* Directed
* Undirected - Bidirectional or No directed.
* Neighbors: Connected by edge

## Storage of Data

* Adjacency List (List of List)
* Adjacency Matrix
* EdgeList
* 2D Matrix (Implicit Graph)

### Adjacency List

It can be implemented:

* List of List or ArrayList of ArrayList or Array of ArrayList
* Hashmap

This is because the size of the adjacency list is fixed at number of vertices that are required in the graphs. The edges can grow. Hence, array or arrayList makes sense. If the number of vertices can grow as well, we can use arraylist of arraylist.

This is an optimal data structure for questions like "Finding neighbors: Time complexity: O(neighbors)".

### Adjacency Matrix

We construct a Vertices * Vertices, where edges are represented by 0 or 1 for an unweighted graph and 0 or weight for a weighted graph

| v     | 0 | 1 | 2 | 3 |
|-------|---|---|---|---|
| **0** | 0 | 1 | 0 | 1 |
| **1** | 0 | 0 | 0 | 1 |
| **2** | 0 | 1 | 0 | 1 |
| **3** | 0 | 1 | 0 | 1 |

Time Complexity: For Neighbors - O(Vertices)
Space: O(V^2)

### EdgeList

This data structure is where we save the edge and its weight in the list. This is preferred where edges are sorted - for example, Minimum spanning tree (MST)

### Implicit Graph

The 2D matrix provided is the graph itself. Can be used in finding the shortest distance. Or, in flood fill algorithm.

## Graph Traversals

### Breadth First Search (BFS)

* Define the starting point
* Go to immediate neighbors first
* Concept of Binary Tree will be useful
* Can be implemented using queue (works as FIFO). We add at rear and remove from front in queue.
* We also use an array, which tracks the visited vertex. Size of this array is same as number of vertices. This is initialized with false and is turned True when the particular vertex has been visited. This is better than adding the visited vertices and checking if the next to visit is already in the queue, which may have O(visited-vertices) time complexity.

Algo:

1. Add the starting node in queue.
2. Pop it out into curr_vertex variable. If this vertex is not visited, then follow 3, 4, 5
3. Print or take appropriate action
4. Mark visited[curr] = True
5. Add neighbors of current vertices into queue.
6. Loop till queue is empty.

Time Complexity: O(V+E)

> Please note that we can have disconnected networks within graph. In such cases, we need to loop through all the vertices to ensure that they have been visited. It still does not change the time complexity.

### Depth First Search (DFS)

1. Keep going to the first neighbor successively.
2. Recursively look for next
3. We will use stack here (works as LIFO). We keep popping out when we get the vertex which is already visited.
4. Print or take appropriate action
5. Mark visited[curr] = True
6. Add neighbors of current vertices into queue.

Time Complexity: O(V+E)

## Topological Sorting

* Directed Acyclic Graph (DAG) is a directed graph with no cycles.
* Topological sorting is used only for DAGs (not for non-DAGs)
* It is a linear order of vertices such that every directed edge u -> v, the vertex u comes before v in the order.

This is a great way to show the dependency clearly.

It also uses DFS for sorting (of course, slightly modified). We just need to track the order. We can use a stack (which LIFO) for tracking which vertex needs to come first and which one needs to come later.

## Cycles in Graph

For undirected graph, we can use DFS / BFS or DSU (Disjoint set union).

For directed graph, we can use DFS / BFS / Topological sorting ([Kahn's algorithm](../Algorithms/Kahns-algorithm.md)) / Graph coloring

Let's learn here using DFS (which is recursion based)

Cycle condition: This is the condition at which the program can confirm that a cycle exists. This happens when the destination of a vertex is already visited, but has not been visited via the current vertex. This is technically called back edge. So a vertex can have three kind of neighbours:

1. Which has been visited, but not its parent (traversal neighbor did not lead to this vertex).
2. Which has been visited, and is its parent
3. Which has not visited

Neighbor 1 is the condition of cycle - so return true.
Neighbor 2 is to do nothing
Neighbor 3 - mark for visit (and capture its candidate - if this returns true, this vertex should return true as well)

## Shortest path algo

Weighted graph: We need to find the overall weight to reach from vertex A to B. There are many applications of this algorithm - like Google map, etc. There are many algorithms to solve this. Few of the most important ones are:

* [Dijksta's](../Algorithms/Dijkstra-Algorithm.md) Algorithm
* [Bellman Ford](../Algorithms/BellmanFord-Algorithm.md) Algorithm

## Minimum Spanning Tree

A minimum spanning tree (MST) or minimum weight spanning tree is a subset of the edges of a connected, edge-weighted undirected graph that connects all the vertices together, without any cycle and with the minimum possible total edge weight.

* MST presumes that graphs are
  * undirected
  * connected
  * weighted
* Weight of the graph is the sum of weight of all edges. MST needs to connect all the vertices without creating a cycle and with minimum possible total weight.
* Since there is no cycle is MST, we can call it a tree as well
* It is a subset of the graph

We use [Prim's](../Algorithms/Prim-Algorithm.md) algorithm to create MST.

## Strongly connected component

SCC is a component in which we can reach every vertex of the component from every other vertex in that component. This can be a single vertex component as well. To do this, we use an algorithm called [Kosaraju's](../Algorithms/Kosaraju-Algorithm.md) algorithm.

This component presumes a directed graph. A undirected graph is strongly connected by the definition of SCC.

## Bridge in Graphs

Bridge is an edge which deletion increases the graph's number of connected components. To find such bridges, we use an algorithm called [Tarjan's](../Algorithms/Tarjan-Algorithm.md) Algorithm

## Articulation Point

A vertex in an undirected connected graph is an articulation point (or cut vertex) if removing it (and edges through it) disconnects the graph.
A graph can have multiple articulation point. This articulation point is used to find vulnerability in a network.
To find this component, we can use [Tarjan's](../Algorithms/Tarjan-Algorithm.md) Algorithm.

Ancestor: A node A that was discovered before curr_node in DFS is the ancestor of curr.

A vertex is articulation point (AP) if -

1. Vertex is a corner point (meaning parent node for this vertex is -1 - starting point of DFS) and its children if are disconnected are greater than 1.
2. If there is only one path to reach to V via U, then U is a AP. This would imply that there is no back-edge from V to U - in such case, V is never an ancestor of U. V can ONLY be visited via U.
3. Vertex U is root of a cycle.

> Please note children in this case is when the destination (or neighbor) is not already visited. If it is already visited, it must have been so via back edge and is actually its ancestor.

## Use cases

* Cycle Detection: If a vertex to be visited is present in recursion stack (implemented as array, not stack - since traveling through stack has higher time complexity).
* Shortest path algorithm:
