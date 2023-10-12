# Hashmap

Hash maps are indexed data structures. A hash map makes use of a hash function to compute an index with a key into an array of buckets or slots. Its value is mapped to the bucket with the corresponding index. The key is unique and immutable. Think of a hash map as a cabinet having drawers with labels for the things stored in them.

[Hash function](Hashes.md) is the core of implementing a hash map. It takes in the key and translates it to the index of a bucket in the bucket list. Ideal hashing should produce a different index for each key. However, collisions can occur. When hashing gives an existing index, we can simply use a bucket for multiple values by appending a list or by rehashing.

In Python, dictionaries are examples of hash maps.

HashMap in Java implements Serializable, Cloneable, Map<K, V> interfaces.Java HashMap extends AbstractMap<K, V> class. It is unsynchronized. It allows to store the null keys as well, but there should be only one null key object and there can be any number of null values. This class makes no guarantees as to the order of the map. To use this class and its methods, you need to import java.util.HashMap package or its superclass.

## Internal working of Hashmap

Internally HashMap contains an array of Node and a node is represented as a class that contains 4 fields:

A **Node <K, V>** -

* int hash
* K key
* V value
* Node<K, V> next

It can be seen that the node is containing a reference to its own object. So it’s a linked list.

## Performance of HashMap

The performance of HashMap depends on 2 parameters which are named as follows:

* **Initial Capacity**: It is the capacity of HashMap at the time of its creation (It is the number of buckets a HashMap can hold when the HashMap is instantiated). In java, it is 2^4=16 initially, meaning it can hold 16 key-value pairs. In Python, it starts with 2^3=8 initially.
* **Load Factor**: It is the percent value of the capacity after which the capacity of Hashmap is to be increased (It is the percentage fill of buckets after which Rehashing takes place). In java, it is 0.75f by default, meaning the rehashing takes place after filling 75% of the capacity. In Python, this is then resized by doubling the number of entries whenever its capacity is reached. This sums up to at least 12 bytes on a 32bit machine and 24 bytes on a 64bit machine.
* **Threshold**: It is the product of Load Factor and Initial Capacity. In java, by default, it is (16 * 0.75 = 12). That is, Rehashing takes place after inserting 12 key-value pairs into the HashMap.
* **Rehashing**: It is the process of doubling the capacity of the HashMap after it reaches its Threshold. In java, HashMap continues to rehash(by default) in the following sequence – 2^4, 2^5, 2^6, 2^7, …. so on.

By keeping the initial capacity higher increases the time complexity of iteration. So it should be chosen very cleverly to increase performance. The expected number of values should be taken into account to set the initial capacity. The most generally preferred load factor value is 0.75 which provides a good deal between time and space costs. The load factor’s value varies between 0 and 1.

> Note: From Java 8 onward, Java has started using Self Balancing BST instead of a linked list for chaining. The advantage of self-balancing bst is, we get the worst case (when every key maps to the same slot) search time is O(Log n).

## Different version of Hashmaps

### Python

https://www.geeksforgeeks.org/hash-map-in-python/

### Java

## Hashset

Set does not allow duplicates

* Insert/Add - O(1)
* Search/Contains - O(1)
* Delete/Remove - O(1)
