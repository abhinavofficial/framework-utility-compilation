# Tree

https://www.geeksforgeeks.org/binary-tree-data-structure/

## Binary Tree Traversal
* Preorder Traversal: Time Complexity: O(n)
  * Root
  * Left Subtree
  * Right Subtree
* Inorder Traversal: Time Complexity: O(n)
  * Left Subtree
  * Root
  * Right Subtree
* Postorder Traversal: Time Complexity: O(n)
  * Left Subtree
  * Right Subtree
  * Root
* Level Traversal: Time Complexity: O(n)
  * Level 1 -> Level n

## Binary Search Tree
* It is a binary tree
* Left subtree nodes < root
* Right subtree nodes > root
* Left and Right subtrees are also BST
* BST can have duplicates, but for general understanding lets assume that there are none unless specially conditioned.
* Special Property: Inorder (LS, root, RS) Traversal of BST gives a **sorted** sequence. 
* Search: O(H)
  * Compare with root.
  * If smaller, search in left subtree
  * If greater, search in right subtree
  * If same, return root

### Types of BST
* In perfectly balanced BST, O(H) would have become log(n)
* In skewed BST , O(H) would have become O(n)

### Strategy
Most problems will be solved by using recursion, i.e. by dividing into sub-problems & making recursive calls on subtrees. Finally, we may need to some calculation on the result returned by these subtrees to the root to solve the problem.

Inorder successor of a node is left-most node in right subtree.

## Binary Tree and Heap
A Heap is a special Tree-based data structure in which the tree is a complete binary tree. For more, read through [Heap](Heaps.md)
