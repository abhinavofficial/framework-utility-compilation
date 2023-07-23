# Dynamic Connectivity

A set of algorithms for solving the so-called dynamic connectivity problem (also, known as union-find problem), the model for which is below:

## Details

Given a set of N objects:
* **Union command**: connect two objects.
* **Find/connected query**: Is there a path connecting the two objects?

This can be applied where manipulating objects are required:
* Elements in a social network.
* Friends in social network.
* etc.

When programming, convenient to name objects 0 to N-1.
* Use integers as array index
* Suppress details not relevant to union-find problem

### Modeling the connection
We assume "is connected to" is an equivalence relation:
* Reflexive: **p** is connected to **p**
* Symmetric: If **p** is connected to **q**, then **q** is connected to **p**.
* Transitive: If **p** is connected to **q** and **q** is connected to **r**, then **p** is connected to **r**.

**Connected components**: Maximal _set_ of objects that are mutually connected.

## Union-find data type (API)
**Goal:** Design efficient data structure for union-find
* Number of objects N can be huge.
* Number of objects M can be huge.
* Find queries and union commands may be intermixed.

```scala
class UF(N: Int) {
  def apply: Unit
  def union(p: Int, q: Int): Unit
  def is_connected(p: Int, q: Int): Boolean
  def find(p: Int): Int
  def count(): Int
}
```

We'll look at two classic algorithms.

## Quick Find
**Implementation**: Check if two objects are the same component.

This is also an eager approach.

### Data Structure
* Integer array id[] of size N.
* Interpretation: p and q are connected iff they have the same id.
* Find: Check id p and q have the same id. ```id(p) = id(q)```

Here, find is quick. Union is slow.

## Quick Union
**Implementation**: Replace components containing two objects with their union. This is the lazy approach to problem.

### Data Structure
* Integer array id[] of size N.
* Interpretation: id(i) is parent of i.
* Root of i is id(id(id(...id(i)...)))
* Union: To merge components containing p and q, set the id of p's root to id of q's root.

Here, union is quick.


### Improvement 1: Weighted quick-union
Instead of set the id of p's root to id of q's root, we can add the small sized component root to the larger one. This would restrict the lean tall tree being formed.

Proposition: Depth of any node x is at most log N
Pf: When does the depth of x increase?
Increases by 1 when tree Ti containing x is merged into another tree Tj.
* The size of the tree containing x is at least double since |Tj| >= |Ti|
* Size of tree containing x can double at most lg N times.

### Further improvement: Weighted quick-union with path compression (WQUPC)
Just after computing the root of p, set the id of each examined node to point to that root.

Proposition: [Hopcroft-Ulman, Tarjan] Starting from an empty data structure, any sequence of M union-find ops on N object makes <= ```c(N + M lg* N)``` array access. lg* is called Iterate log function.
* Analysis can be improved to ```N + M f(M, N)```

This is so close to being linear in practice, though not in theory.

## Cost Model
Based on array accesses (for read and write)

| Algorithm  | Initialize | Union | Find |
|------------|------------|-------|------|
| quick-find | N          | N     | 1    |
| quick-find | N          | N     | 1    |

Union operation for N -> N^2

Rough standard for now:
* ```10^9``` operations per sec
* ```10^9``` words of main memory


| Algorithm                                  | Initialize  | 
|--------------------------------------------|-------------|
| quick-find                                 | M N         |
| quick-union                                | M N         |
| weighted quick-union                       | N + M log N |
| quick-union with path compression          | N + M log N |
| weighted quick-union with path compression | N + M lg* N |