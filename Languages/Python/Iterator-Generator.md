# Iterator and Generator

## Iterable
Is structure or collection which can be iterated over like `for x in xs:` list, string, set, dict are all iterable.

```python
from collections.abc import Iterable

isinstance(xs, Iterable) # if returns true
```

subscriptable: It basically means that the object implements the __getitem__() method. In other words, it describes objects that are "containers", meaning they contain other objects. This includes strings, lists, tuples, and dictionaries. However, set is not subscriptable. So, if you fire hasattr(on_set_object, '__getitem__') would return false.

In Python as you could note that string is a sequence. So, if you try to get a random choice via Python's random module, it would work just fine. However, if you try to do the same with numpy.random module, it would throw an error. Data Science tools strings to be an element. This is because, it does not consider str to be further broken for its purpose.

```python
from random import Random
rnd = Random(0)
s = 'abc'
print(f"{rnd.choice(s) = }")

from numpy.random import default_rng
rng = default_rng(0)
s = 'abc'
print(f"{rng.choice(s) = }")
# ValueError: a must be a sequence or an integer, not <class 'str'>

s = [*'abc']
print(f"{rng.choice(s) = }") # This works just fine now
```

Hence, we can define iterable as just some data which can be broken down into parts or is a collection of elements. Any custom class, if implements an __iter__ method, it would be treated as Iterator

```python
from collections.abc import Iterable
class T:
    def __iter__(self): pass
print(f"{isinstance(T(), Iterable) = }") # return true
```

Iterable can be
* concrete -> the data that we are iterating over is physically present in memory
* abstract -> the data that we are iterating over is constructed on demand

lazy / eager

materializing is a process to convert abstract into concrete.
```python
from sys import set_int_max_str_digits
set_int_max_str_digits(1000000)
print(range(2 ** 20000))

range(20) # abstract
xs = [*range(10)] # concrete
xs = list(range(10)) # concrete
```


## Iterator
Iterator is an object which allows a programmer to traverse through all the elements of a collection, regardless of its specific implementation. It has reference to the data and some state (where we last left off). In Python, an iterator object implements two methods, __iter__ and __next__. When there is no element to iterate, we can throw a StopIteration error. See below -
```python
class ListIterator:
    def __init__(self, data):
        self.data = data
        self.state = 0
    def __iter__(self):
        return  self
    def __next__(self):
        try:
            rv = self.data[self.state]
            self.state += 1
            return rv
        except IndexError as e:
            raise StopIteration() from e
```

This is already implemented in itertools, exposing an iterator object which implements two methods, iter() and next().

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

> Comprehension syntax is more restricted than regular for-loop syntax.
> 1. Restrictive in grammar: [expr for loopvar in looptarget if pred]. This expr must be a single Python expression. Please see below
```python
def f(): pass # statement
f = lambda: None # expression

class T: pass # statement
T = type('T', (), {}) # expression

x = 123 # statement
x := 123 # expression

match or try-catch statements do not have corresponding expression statement
```

> 2. You can refer to a thing while you build in. Think of Scan/fold scenario

map and filter

```python
xs = [-3, -2, -1, 0, 1, 2, 3]

lc = [x*2 for x in xs]
sc = {x*2 for x in xs}
dc = {x: x*2 for x in xs}
fsc = frozenset({x*2 for x in xs})
strc = ''.join([x.upper() for x in "abc"])

# for tuple
tc = (x: x*2 for x in xs) # this returns a generator object
tcomp = *(x: x*2 for x in xs), # this is concrete
```

zip, enumerate,
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

https://realpython.com/introduction-to-python-generators/
