# Heap

A Heap is a special Tree-based data structure in which the tree is a complete binary tree.

## Properties of Heap

Heap has the following Properties:

* Complete Binary Tree: A heap tree is a complete binary tree, meaning all levels of the tree are fully filled except possibly the last level, which is filled from left to right. This property ensures that the tree is efficiently represented using an array.
* Heap Property: This property ensures that the minimum (or maximum) element is always at the root of the tree according to the heap type.
* Parent-Child Relationship: The relationship between a parent node at index ‘i’ and its children is given by the formulas: left child at index 2i+1 and right child at index 2i+2 for 0-based indexing of node numbers.
* Efficient Insertion and Removal: Insertion and removal operations in heap trees are efficient. New elements are inserted at the next available position in the bottom-right most level, and the heap property is restored by comparing the element with its parent and swapping if necessary. Removal of the root element involves replacing it with the last element and heapifying down.
* Efficient Access to Extremal Elements: The minimum or maximum element is always at the root of the heap, allowing constant-time access.

## Operations of Heap Data Structure

### Heapify

It is a process of creating a heap from an array. It is in essence, the process to rearrange the elements to maintain the property of heap data structure. It is done when a certain node creates an imbalance in the heap due to some operations on that node. It takes O(log N) to balance the tree.

* For max-heap, it balances in such a way that the maximum element is the root of that binary tree, and
* For min-heap, it balances in such a way that the minimum element is the root of that binary tree.

### Insertion

It is a process to insert an element in existing heap. Time complexity O(log N).

> If we insert a new element into the heap since we are adding a new element into the heap, so it will distort the properties of the heap. We therefore, need to perform the heapify operation so that it maintains the property of the heap.

### Deletion

Deleting the top element of the heap or the highest priority element, and then organizing the heap and returning the element with time complexity O(log N).

> If we delete the element from the heap it always deletes the root element of the tree and replaces it with the last element of the tree. Since we delete the root element from the heap it will distort the properties of the heap, so we need to perform heapify operations so that it maintains the property of the heap. 

### Peek

to check or find the first (or can say the top) element of the heap.

* getMax (For max-heap) or getMin (For min-heap): It finds the maximum element or minimum element for max-heap and min-heap respectively. and as we know minimum and maximum elements will always be the root node itself for min-heap and max-heap respectively. It takes O(1) time.
* removeMin or removeMax: This operation returns and deletes the maximum element and minimum element from the max-heap and min-heap respectively. In short, it deletes the root element of the heap binary tree. It takes O(1) time.

## Types of Heap

Generally, Heaps can be of many types:

* [Max-Heap](#max-heap)
* [Min-Heap](#min-heap)
* [Binomial Heap](https://www.geeksforgeeks.org/binomial-heap-2/)
* [Fibonacci Heap](https://www.geeksforgeeks.org/fibonacci-heap-set-1-introduction/)
* [Leftist Heap](https://www.geeksforgeeks.org/leftist-tree-leftist-heap/)
* [K-ary Heap](https://www.geeksforgeeks.org/k-ary-heap/?ref=lbp)

### Max Heap

In a Max-Heap, the key present at the root node must be greatest among the keys present at all of its children. The same property must be recursively true for all subtrees in that Binary Tree.

The total number of comparisons required in the max heap is according to the height of the tree. The height of the complete binary tree is always logn; therefore, the time complexity would also be O(logn).

### Min Heap

In a Min-Heap, the key present at the root node must be minimum among the keys present at all of its children. The same property must be recursively true for all subtrees in that Binary Tree.

The total number of comparisons required in the min heap is according to the height of the tree. The height of the complete binary tree is always logn; therefore, the time complexity would also be O(logn).

### Binomial Heap

### Fibonacci Heap

### Leftist Heap

### K-ary Heap
