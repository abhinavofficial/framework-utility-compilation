# Stacks
LIFO

## Operation
* Push - O(1): Add the element at the top
* Pop - O(1): Remove the top out
* Peek - O(1) - Look into the top

## Implementation

### Array
Array is fixed sized. Think of array as vertical where the 0th element is at the bottom of the stack. This is slightly hectic since there may be user impact when the stack is full.

### ArrayList
Array is variable sized. Think of arraylist as vertical where the 0th element is at the bottom of the stack.

### Linked List
Array is variable sized. Each time we add or remove an element, it should become the head. It is therefore always of O(1) complexity.
