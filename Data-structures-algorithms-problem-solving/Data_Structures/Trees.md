# Tree

https://www.geeksforgeeks.org/binary-tree-data-structure/

## Binary Tree

### Binary Tree Traversal

* Pre-order Traversal: Time Complexity: O(n)
  * Root
  * Left Subtree
  * Right Subtree
* In-order Traversal: Time Complexity: O(n)
  * Left Subtree
  * Root
  * Right Subtree
* Post-order Traversal: Time Complexity: O(n)
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

### Traversal of BST

The traversals are same as Binary Tree.

### Types of BST

* In perfectly balanced BST, O(H) would have become log(n)
* In skewed BST , O(H) would have become O(n)

### Self Balancing tree - AVL Tree

Named after its inventor, Georgy Adelson-Velsky and Evgenii Landis, published in their paper "An algorithm for the organization of information"

An AVL tree defined as a self-balancing Binary Search Tree (BST) where the difference between heights of left and right subtrees for any node cannot be more than one. This difference is known as balance factor, BF `(H(L) - H(R)) must be [-1, 0, 1]`.

To balance the tree, it may rotate in one of the following four ways:

* Left rotation: When a node is added into the right subtree of the right subtree, if the tree gets out of balance, we do a single left rotation.
* Right rotation: If a node is added to the left subtree of the left subtree, the AVL tree may get out of balance, we do a single right rotation.
* Left-right rotation: A left-right rotation is a combination in which first left rotation takes place after that right rotation executes.
* Right-left rotation: A right-left rotation is a combination in which first right rotation takes place after that left rotation executes.

Implementation:

* Perform the normal BST insertion.
* The current node must be one of the ancestors of the newly inserted node. Update the height of the current node.
* Get the balance factor (left subtree height – right subtree height) of the current node.
* If the balance factor is greater than 1, then the current node is unbalanced, then we are either in the Left-Left case or left-Right case. To check whether it is Left-Left case or not, compare the newly inserted key with the key in the left subtree root.
* If the balance factor is less than -1, then the current node is unbalanced, then we are either in the Right-Right case or Right-Left case. To check whether it is the Right-Right case or not, compare the newly inserted key with the key in the right subtree root.

* Applications of AVL Tree:
  * It is used to index huge records in a database and also to efficiently search in that.
  * For all types of in-memory collections, including sets and dictionaries, AVL Trees are used.
  * Database applications, where insertions and deletions are less common but frequent data lookups are necessary.
  * Software that needs optimized search.
  * It is applied in corporate areas and storyline games.
* Advantages of AVL Tree:
  * AVL trees can self-balance themselves.
  * It is surely not skewed.
  * It provides faster lookups than Red-Black Trees.
  * Better searching time complexity compared to other trees like binary tree.
  * Height cannot exceed log(N), where, N is the total number of nodes in the tree.
* Disadvantages of AVL Tree:
  * It is difficult to implement.
  * It has high constant factors for some of the operations.
  * Less used compared to Red-Black trees.
  * Due to its rather strict balance, AVL trees provide complicated insertion and removal operations as more rotations are performed.
  * Take more processing for balancing.

### Self Balancing tree - Red-Black Tree

Red Black Trees are self-balancing, meaning that the tree adjusts itself automatically after each insertion or deletion operation. It uses a simple but powerful mechanism to maintain balance, by coloring each node in the tree either red or black. It has a good efficient worst case running time complexity.

**Properties of Red Black Tree**:

The Red-Black tree satisfies all the properties of binary search tree in addition to that it satisfies following additional properties –

1. Root property: The root is black.
2. External property: Every leaf (Leaf is a NULL child of a node) is black in Red-Black tree.
3. Internal property: The children of a red node are black. It is possible that parent of red node is a black node.
4. Depth property: All the leaves have the same black depth.
5. Path property: Every simple path from root to descendant leaf node contains same number of black nodes.

The result of all these above-mentioned properties is that the Red-Black tree is roughly balanced.

Rules That Every Red-Black Tree Follows:

1. Every node has a color either red or black.
2. The root of the tree is always black.
3. There are no two adjacent red nodes (A red node cannot have a red parent or red child).
4. Every path from a node (including root) to any of its descendants NULL nodes has the same number of black nodes.
5. Every leaf (e.i. NULL node) must be colored BLACK.

**Why Red-Black Trees?**

Most of the BST operations (e.g., search, max, min, insert, delete.. etc) take O(h) time where h is the height of the BST. The cost of these operations may become O(n) for a skewed Binary tree. If we make sure that the height of the tree remains O(log n) after every insertion and deletion, then we can guarantee an upper bound of O(log n) for all these operations. The height of a Red-Black tree is always O(log n) where n is the number of nodes in the tree.

* The black height of the red-black tree is the number of black nodes on a path from the root node to a leaf node. Leaf nodes are also counted as black nodes. So, a red-black tree of height h has black height >= h/2.
* Height of a red-black tree with n nodes is h<= 2 log2(n + 1).
* All leaves (NIL) are black.
* The black depth of a node is defined as the number of black nodes from the root to that node i.e the number of black ancestors.
* Every red-black tree is a special case of a binary tree.

Search implementation is same as BST.

## B-Tree

Unlike traditional binary search trees, B-Trees are characterized by the large number of keys that they can store in a single node, which is why they are also known as “large key” trees. Each node in a B-Tree can contain multiple keys, which allows the tree to have a larger branching factor and thus a shallower height. This shallow height leads to less disk I/O, which results in faster search and insertion operations. B-Trees are particularly well suited for storage systems that have slow, bulky data access such as hard drives, flash memory, and CD-ROMs.

B-Trees maintain balance by ensuring that each node has a minimum number of keys, so the tree is always balanced. This balance guarantees that the time complexity for operations such as insertion, deletion, and searching is always O(log n), regardless of the initial shape of the tree.

Properties of B-Tree:

* B-Tree is defined by the term minimum degree ‘t‘. The value of ‘t‘ depends upon disk block size. This degree is sometimes called order.
* Like other balanced Binary Search Trees, the time complexity to search, insert and delete is O(log n).

* All leaves are at the same level.
* Every node except the root ~~must contain at least~~ can contain up to t-1 keys. Every node must have at `ceil(t/2)` before creating new node. The root may contain a minimum of 1 key.
* All nodes (including root) may contain at most (2*t – 1) keys `2*number of key of children + 1 => 2(t-1) + 1`.
* Number of children of a node is equal to the number of keys in it plus 1. Or, simply, number of children of a node can be t.
* All keys of a node are sorted in increasing order. The child between two keys k1 and k2 contains all keys in the range from k1 and k2.
* Insertion of a Node in B-Tree happens only at Leaf Node. B-Tree grows and shrinks from the root which is unlike Binary Search Tree. Binary Search Trees grow downward and also shrink from downward.

## Strategy

Most problems will be solved by using recursion, i.e. by dividing into sub-problems & making recursive calls on subtrees. Finally, we may need to some calculation on the result returned by these subtrees to the root to solve the problem.

In-order successor of a node is left-most node in right subtree.

We can also use stacks.

## Binary Tree and Heap

A Heap is a special Tree-based data structure in which the tree is a complete binary tree. For more, read through [Heap](Heaps.md)
