# Iterator and Generator

## Iterator
Iterator is an object which allows a programmer to traverse through all the elements of a collection, regardless of its specific implementation. In Python, an iterator object implements two methods, iter() and next().

String, List or Tuple objects can be used to create an Iterator.

```python
#!/usr/bin/python3
import sys

list = [1,2,3,4]
it = iter(list) # this builds an iterator object
print(next(it)) #prints next available element in iterator
#Iterator object can be traversed using regular for statement
for x in it:
    print (x, end=" ")
#or using next() function
while True:
    try:
        print(next(it))
    except StopIteration:
        sys.exit()
```

## Generator
A generator is a function which returns a generator iterator - that produces or yields a sequence of values using yield method. It looks like a normal function except that it contains yield expressions for producing a series of values usable in a for-loop or that can be retrieved one at a time with the next() function. Usually refers to a generator function, but may refer to a generator iterator in some contexts. In cases where the intended meaning isn’t clear, using the full terms avoids ambiguity.

When a generator function is called, it returns a generator object without even beginning execution of the function. When the next() method is called for the first time, the function starts executing until it reaches the yield statement, which returns the yielded value. The yield keeps track i.e. remembers the last execution and the second next() call continues from previous value.


The following example defines a generator, which generates an iterator for all the Fibonacci numbers.
```python
#!usr/bin/python3

import sys
def fibonacci(n): #generator function
    a, b, counter = 0, 1, 0
    while True:
        if (counter > n):
            return
        yield a
        a, b = b, a + b
        counter += 1

f = fibonacci(5) #f is iterator object
while True:
    try:
        print (next(f), end=" ")
    except StopIteration:
        sys.exit()
```

More on [generator](https://docs.python.org/3.10/reference/expressions.html#generator-expressions) and [yield](https://docs.python.org/3.10/reference/expressions.html#generator-expressions) expressions.
