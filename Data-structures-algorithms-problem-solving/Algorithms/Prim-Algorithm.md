# Prim's Algorithm

## Algorithm
1. Let's assume there are two sets: MST Set and non-MST sets.
2. Initially all the vertices are in non-MST set and total weight = 0 
3. Move the starting vertices in MST set and remove from non-MST set 
4. Find node whose weight from this node is minimum. 
5. Move this node and edge to MST and remove from non-MST and update the total weight. 
6. Now find out which of the vertices of MST set has the minimum weight to the non-MST set.
7. Move this node and edge to MST and remove from non-MST and update the total weight.
8. Repeat 6 and 7 till there is none left in non-MST set.

## Implementation
Hint:
Since we need to find the minimum weight edge, probably priority queue is the best way to implement for non-MST.
We can use array (visited) to track nodes in MST.

Time Complexity: O(E* logE)

Implementation is present at https://github.com/abhinavofficial/prep-inteview/blob/main/Graph.py