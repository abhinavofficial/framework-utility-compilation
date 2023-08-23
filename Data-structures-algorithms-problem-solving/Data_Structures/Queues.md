# Queue

Front -> Rear

## Operation
* Enque: Addition in the rear
* Dequeue: Pop from the front
* Front: peek

FIFO

## Implementation

### Array
Array is fixed sized. This is slightly hectic since there may be user impact when the stack is full. We should keep both front and rear to track. Front is always 0th element, but rear can keep moving as new element is added. Remove is O(n), while add and peek will be O(n)

## Circular Queue using Array
While adding -> Rear = (Rear + 1) % size
While removing -> Front = (Front + 1) % size

All - add, remove and peek will be O(1)

### ArrayList
Array is variable sized.

### Linked List
Linked List is variable sized. Each time we add or remove an element, it should become the head. It is therefore always of O(1) complexity.


> Please do not get confused with deque. This is a double ended queue.

