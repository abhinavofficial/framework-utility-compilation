# Graphs
Graph is a network of nodes. 

## Key concepts
* Vertex: Node
* Edge: source, destination, direction (optional) and weight (optional)
* Weight: 
* Unweighted
* Directed: 
* Undirected: Bidirectional or No directed.
* Neighbours: Connected by edge

## Storage of Data
* Adjacency List (List of List)
* Adjacency Matrix
* EdgeList
* 2D Matrix (Implicit Graph)

## Adjacency List
It can be implemented:
* List of List or ArrayList of ArrayList or Array of ArrayList
* Hashmap

This is because the size of the adjacency list is fixed at number of vertices that are required in the graphs. The edges can grow. Hence, array or arrayList makes sense. If the number of vertices can grow as well, we can use arraylist of arraylist.

This is an optimal data structure for questions like 
* Finding neighbours: Time complexity: O(neighbours)


## Adjacency Matrix
We construct a Vertices * Vertixes, where edges are represented by 0 or 1 for an unweighted graph and 0 or weight for a weighted graph

| v     | 0 | 1 | 2 | 3 |
|-------|---|---|---|---|
| **0** | 0 | 1 | 0 | 1 |
| **1** | 0 | 0 | 0 | 1 |
| **2** | 0 | 1 | 0 | 1 |
| **3** | 0 | 1 | 0 | 1 |

Time Complexity: For Neighbours - O(Vertices)
Space: O(V^2)

## EdgeList
This data structure is where we save the edge and its weight in the list. This is preferred where edges are sorted - for example, Minimum spanning tree (MST)

## Implicit Graph
The 2D matrix provided is the graph itself. Can be used in finding the shortest distance. Or, in flood fill algorithm.