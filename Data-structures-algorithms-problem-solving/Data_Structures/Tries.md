# Trie Data Structure
Also known as prefix tree, digital search or retrieval tree or Patricia trie or radix tree

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

All prefixes of all suffixes or all suffixes of all prefixes are the substring of a string.

Some of the worked out example of how to implement a Trie is shown at https://github.com/abhinavofficial/prep-inteview/blob/main/Trie.py

In Python, some of these implementation is prebuilt in module pytrie. It exposes basic Trie, SortedTrie, StringTrie and SortedStringTrie with other useful utility functions.

```python
# Function which returns all strings
# that contains given prefix
from pytrie import StringTrie
 
def prefixSearch(arr,prefix):
     
    # create empty trie
    trie=StringTrie()
 
    # traverse through list of strings
    # to insert it in trie. Here value of
    # key is itself key because at last
    # we need to return
    for key in arr:
        trie[key] = key
 
    # values(search) method returns list
    # of values of keys which contains
    # search pattern as prefix
    return trie.values(prefix)
 
# Driver program
if __name__ == "__main__":
    arr = ['geeksforgeeks','forgeeks','geeks','eeksfor']
    prefix = 'geek'
    output = prefixSearch(arr,prefix)
    if len(output) > 0:
        print (output)
    else:
        print ('Pattern not found')
```

## Types of trie
## Standard Trie 
A standard trie have the following properties:
* A Standard Trie has the below structure:
```python
class Node:
       def __init__(self):
            # Array to store the nodes of a tree
            children: [Node] = [None]*26
            # To check for end of string
            isWordEnd: bool = False
```
* It is an ordered tree like data structure.
* Each node(except the root node) in a standard trie is labeled with a character.
* The children of a node are in alphabetical order.
* Each node or branch represents a possible character of keys or words.
* Each node or branch may have multiple branches. 
* The last node of every key or word is used to mark the end of word or node.

## Compressed Trie
A Compressed trie have the following properties:
* A Compressed Trie has the below structure:
```java
class Node {

   // Array to store the nodes of tree
   Node[] children = new Node[26];

   // To store the edgeLabel
   StringBuilder[] edgeLabel = new StringBuilder[26];

   // To check for end of string
   boolean isEnd;
}
```
*  A Compressed Trie is an advanced version of the standard trie.
*  Each node (except the leaf nodes) has at least 2 children.
*  It is used to achieve space optimization.
*  To derive a Compressed Trie from a Standard Trie, compression of chains of redundant nodes is performed.
*  It consists of grouping, re-grouping and un-grouping of keys of characters.
*  While performing the insertion operation, it may be required to un-group the already grouped characters.
*  While performing the deletion operation, it may be required to re-group the already grouped characters.
*  A compressed trie T storing s strings(keys) has s external nodes and O(s) total number of nodes.

## Suffix trie
* A suffix Trie has the below structure:
```text
struct SuffixTreeNode {
    // Array to store the nodes
    struct SuffixTreeNode *children[256]; 
   
    //pointer to other node via suffix link 
    struct SuffixTreeNode *suffixLink; 
   
    // (start, end) interval specifies the edge,
    // by which the node is connected to its 
    // parent node
    int start; 
    int *end; 
   
    // For leaf nodes, it stores the index of 
    // Suffix for the path  from root to leaf
    int suffixIndex; 
}
```
* A Suffix Trie is an advanced version of the compressed trie.
* The most common application of suffix trie is Pattern Matching.
* While performing the insertion operation, both the word and its suffixes are stored.
* A suffix trie is also used in word matching and prefix matching.
* To generate a suffix trie, all the suffixes of given string are considered as individual words.
* Using the suffixes, compressed trie is built.

https://python.plainenglish.io/trie-data-structure-and-the-implementation-of-its-methods-with-type-annotations-in-python-1737ba25adc4

https://towardsdatascience.com/memoization-in-python-57c0a738179a

https://www.geeksforgeeks.org/pattern-searching-using-suffix-tree/