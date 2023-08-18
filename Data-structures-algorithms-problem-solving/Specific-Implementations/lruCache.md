
```python3
from typing import Generic, TypeVar

T = TypeVar("T")
U = TypeVar("U")

class DoubleLinkedListNode(Generic[T, U]):
    def __init__(self, key: T | None, val: U | None):
        self.key = key
        self.val = val
        self.prev: DoubleLinkedListNode[T, U] | None = None
        self.next: DoubleLinkedListNode[T, U] | None = None


class DoubleLinkedList(Generic[T, U]):
    def __init__(self):
        self.head: DoubleLinkedListNode[T, U] = DoubleLinkedListNode(None, None)
        self.tail: DoubleLinkedListNode[T, U] = DoubleLinkedListNode(None, None)
        self.head.next, self.tail.prev  = self.tail, self.head

    def addNodeNextToHead(self, node: DoubleLinkedListNode[T, U]):
        oldNode: DoubleLinkedListNode[T, U] = self.head.next
        oldNode.prev = node
        node.prev = self.head
        self.head.next = node
        node.next = oldNode
        return node

    def remove(self, node: DoubleLinkedListNode[T, U]):
        oldPrevNode: DoubleLinkedListNode[T, U] | None = node.prev
        oldNextNode: DoubleLinkedListNode[T, U] | None = node.next

        oldPrevNode.next = oldNextNode
        oldNextNode.prev = oldPrevNode


    def __repr__(self) -> str:
        rep = ["DoubleLinkedList"]
        node = self.head
        while node.next is not None:
            rep.append(str(node.key))
            node = node.next
        rep.append(str(self.tail.key))
        return ",\n    ".join(rep)



class LRUCache(Generic[T, U]):
    TTL: int = 10_000 # in ms

    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self.list: DoubleLinkedList[T, U] = DoubleLinkedList()
        self.cache: dict[T, DoubleLinkedListNode[T, U]] = {}

    def get(self, key: T) -> U | int:
        if not self.__contains(key):
            return -1

        # get the node from cache
        node: DoubleLinkedListNode[T, U] = self.cache[key]
        # remove the old reference from the list
        self.list.remove(node)
        # add new node in the list
        self.list.addNodeNextToHead(node)

        return node.val

    def put(self, key: T, value: U) -> None:
        node = DoubleLinkedListNode(key=key, val=value)
        if self.__contains(key=key):
            existingNode = self.cache[key]
            self.list.remove(existingNode)
            self.__addNodeOnCache(node)
            return


        if len(self.cache) >= self.capacity: # remove LRU node
            lruNode = self.list.tail.prev
            self.list.remove(lruNode)
            self.cache.pop(lruNode.key)

        self.__addNodeOnCache(node)


    def __contains(self, key: T) -> bool:
        return key in self.cache

    def __addNodeOnCache(self, node) -> None:
        self.list.addNodeNextToHead(node)
        self.cache[node.key] =  node



# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)

lruCache = LRUCache(3)
lruCache.put(1, "Ram")
lruCache.put(2, [1, 2, 3])
lruCache.put(3, 5)
print(lruCache.get(1))
print(lruCache.get(2))
print(lruCache.get(3))
lruCache.put(4, {"10": "4", "6": "5"})
lruCache.put(5, False)

print(lruCache.get(4))
print(lruCache.get(5))
print(lruCache.get(6))
print(lruCache.list)
 ```

```python
from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key: int) -> int:
        value = self.cache.get(key, -1)
        if value != -1:
            self.cache.move_to_end(key)
        return value
    
    def put(self, key: int, value: int) -> None:
        self.cache[key] = value
        self.cache.move_to_end(key)

        if len(self.cache) > self.capacity:
            self.__evict()

    def __evict(self) -> None:
        self.cache.popitem(last=False)

lruCache = LRUCache(3)
print(lruCache.get(1))
lruCache.put(1, 10)
lruCache.put(2, 20)
lruCache.put(3, 30)
print(lruCache.get(2))
lruCache.put(4, 40)
print(lruCache.get(1))
print(lruCache.get(4))
```

```python
from collections import OrderedDict
import time

class LRUCache:
    def __init__(self, capacity: int, ageInS:1):
        self.capacity = capacity
        self.maxAge = ageInS
        self.cache = OrderedDict()

    def get(self, key: int) -> int:
        (value, time) = self.cache.get(key, (-1, None))
        self.put(key, key)
        return value
    
    def put(self, key: int, value: int) -> None:
        self.cache[key] = (value, time.time())
        self.cache.move_to_end(key)

        if len(self.cache) > self.capacity:
            self.__evict()

    def __evict(self) -> None:
        # Remove the expired
        
        self.cache.popitem(last=False)

lruCache = LRUCache(3)
print(lruCache.get(1))
lruCache.put(1, 10)
lruCache.put(2, 20)
lruCache.put(3, 30)
print(lruCache.get(2))
lruCache.put(4, 40)
print(lruCache.get(1))
print(lruCache.get(4))
```
