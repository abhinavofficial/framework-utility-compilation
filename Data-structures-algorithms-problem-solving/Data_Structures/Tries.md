# Trie Data Structure
Also known as prefix tree, digital search or retrieval tree

Prefix - a substring which is starting at 0 and end at any index < N-1
Trie is in essence a tree data structure, in particular a k-ary tree.

Time complexity of Tries is O(L) where L is the length of the word. It is better than AVL or BST. Hence, it is used for faster search.

## Implementation
Points to note
* Root is an empty node.
* Prefix is not repeated. 
* Also save the end of word at the node where the word ends.

1. Take the first word
2. Keep append each character of the word in node. Mark end-of-word as application
3. For the next word, check if the prefix already exists level by level.
   4. If it does, then continue with the next.
   5. If it does not, add to node at that level



https://python.plainenglish.io/trie-data-structure-and-the-implementation-of-its-methods-with-type-annotations-in-python-1737ba25adc4

https://towardsdatascience.com/memoization-in-python-57c0a738179a

https://www.geeksforgeeks.org/prefix-matching-python-using-pytrie-module/

https://www.geeksforgeeks.org/cachetools-module-in-python/

https://leetcode.com/problems/lfu-cache/description/