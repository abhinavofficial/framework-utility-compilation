# Flexible Arguments in Python

*args and **kwargs allow you to pass multiple **positional** arguments or **keyword** arguments to a function. Please note args or kwargs is just a name - you can use any name.

A function would work, even if you pass the iterable object as integers instead of args. All that matters here is that you use the unpacking operator (*). Bear in mind that the iterable object you’ll get using the unpacking operator * is **not a list but a tuple**. A tuple is similar to a list in that they both support slicing and iteration. However, lists are mutable, while tuples are not.
```python
def my_sum(*integers):
    result = 0
    for x in integers:
        result += x
    return result

print(my_sum(1, 2, 3))
```

**kwargs works just like *args, but instead of accepting positional arguments it accepts keyword (or named) arguments. Here the unpacking operator is **. Here the iterable object is a standard dict.
```python
def concatenate(**kwargs):
    result = ""
    # Iterating over the Python kwargs dictionary
    for arg in kwargs.values(): # Note the use of .values
        result += arg
    return result

print(concatenate(a="Real", b="Python", c="Is", d="Great", e="!"))
```

## Ordering Arguments in a Function
**Order matters**. Just as non-default arguments have to precede default arguments, so *args must come before **kwargs.

The correct order for your parameters is:
* Standard arguments
* *args arguments
* **kwargs arguments

## Unpacking With the Asterisk Operators: * & **
The single and double asterisk unpacking operators were introduced in Python 2. As of the 3.5 release, they have become even more powerful, thanks to [PEP 448](https://www.python.org/dev/peps/pep-0448/). In short, the unpacking operators are operators that unpack the values from iterable objects in Python. 
The single asterisk operator **\* can be used on any iterable** that Python provides, while the double asterisk operator **** can only be used on dictionaries**.
```python
# print_unpacked_list.py
my_list = [1, 2, 3]
print(*my_list)
```

> Please note: Instead of a list, print() has taken three separate arguments as the input.

When you use the * operator to unpack a list and pass arguments to a function, it’s exactly as though you’re passing every single argument alone. This means that you can use multiple unpacking operators to get values from several lists and pass them all to a single function.

### Other convenient uses of the unpacking operator
* Say, you need to split a list into three different parts. The output should show the first value, the last value, and all the values in between.
```python
my_list = [1, 2, 3, 4, 5, 6]
a, *b, c = my_list
```
* You can do with the unpacking operator * is to split the items of any iterable object. This could be very useful if you need to merge two lists.
```python
my_first_list = [1, 2, 3]
my_second_list = [4, 5, 6]
my_merged_list = [*my_first_list, *my_second_list]
```
* You can even merge two different dictionaries by using the unpacking operator **
```python
my_first_dict = {"A": 1, "B": 2}
my_second_dict = {"C": 3, "D": 4}
my_merged_dict = {**my_first_dict, **my_second_dict}

print(my_merged_dict)
```
* * operator works on any iterable object. It can also be used to unpack a string:
```python
a = [*"RealPython"]
print(a)

# or
*a, = "RealPython"
print(a)
```