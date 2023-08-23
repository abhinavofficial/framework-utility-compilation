# functools
The functools module is for higher-order functions: functions that act on or return other functions. In general, any callable object can be treated as a function for the purposes of this module.

## @functools.cache(user_function)
Simple lightweight unbounded function cache. Sometimes called “memoize”.

Returns the same as lru_cache(maxsize=None), creating a thin wrapper around a dictionary lookup for the function arguments. Because it never needs to evict old values, this is smaller and faster than lru_cache() with a size limit.

The cache is threadsafe so that the wrapped function can be used in multiple threads. This means that the underlying data structure will remain coherent during concurrent updates.

It is possible for the wrapped function to be called more than once if another thread makes an additional call before the initial call has been completed and cached.

## @functools.cached_property(func)
Transform a method of a class into a property whose value is computed once and then cached as a normal attribute for the life of the instance. Similar to `property()`, with the addition of caching. Useful for expensive computed properties of instances that are otherwise effectively immutable.

If a mutable mapping is not available or if space-efficient key sharing is desired, an effect similar to `cached_property()` can also be achieved by stacking `property()` on top of `lru_cache()`.

## @functools.cmp_to_key(func)
Transform an old-style comparison function to a key function. Used with tools that accept key functions (such as `sorted()`, `min()`, `max()`, `heapq.nlargest()`, `heapq.nsmallest()`, `itertools.groupby()`). This function is primarily used as a transition tool for programs being converted from Python 2 which supported the use of comparison functions.

A comparison function is any callable that accepts two arguments, compares them, and returns a negative number for less-than, zero for equality, or a positive number for greater-than. A key function is a callable that accepts one argument and returns another value to be used as the sort key.

Example: `sorted(iterable, key=cmp_to_key(locale.strcoll))  # locale-aware sort order`

##  @functools.lru_cache(user_function) and @functools.lru_cache(maxsize=128, typed=False)
Decorator to wrap a function with a memoizing callable that saves up to the maxsize most recent calls. It can save time when an expensive or I/O bound function is periodically called with the same arguments.

The cache is threadsafe so that the wrapped function can be used in multiple threads. This means that the underlying data structure will remain coherent during concurrent updates.

It is possible for the wrapped function to be called more than once if another thread makes an additional call before the initial call has been completed and cached.

If maxsize is set to None, the LRU feature is disabled and the cache can grow without bound.

If typed is set to true, function arguments of different types will be cached separately. If typed is false, the implementation will usually regard them as equivalent calls and only cache a single result. (Some types such as str and int may be cached separately even when typed is false.)

Since a dictionary is used to cache results, the positional and keyword arguments to the function must be hashable. An object is hashable if it has a hash value which never changes during its lifetime (it needs a __hash__() method), and can be compared to other objects (it needs an __eq__() method). Hashable objects which compare equal must have the same hash value. Hashability makes an object usable as a dictionary key and a set member, because these data structures use the hash value internally. Most of Python’s immutable built-in objects are hashable; mutable containers (such as lists or dictionaries) are not; immutable containers (such as tuples and frozensets) are only hashable if their elements are hashable. Objects which are instances of user-defined classes are hashable by default. They all compare unequal (except with themselves), and their hash value is derived from their id().

If user_function is specified, it must be a callable. This allows the lru_cache decorator to be applied directly to a user function, leaving the maxsize at its default value of 128.

> Type specificity applies only to the function’s immediate arguments rather than their contents. The scalar arguments, Decimal(42) and Fraction(42) are be treated as distinct calls with distinct results. In contrast, the tuple arguments ('answer', Decimal(42)) and ('answer', Fraction(42)) are treated as equivalent.

To help measure the effectiveness of the cache and tune the maxsize parameter, the wrapped function is instrumented with a `cache_info()` function that returns a named tuple showing _hits_, _misses_, _maxsize_ and _currsize_.

The decorator also provides a `cache_clear()` function for clearing or invalidating the cache.

The cache keeps references to the arguments and return values until they age out of the cache or until the cache is cleared.

## @functools.total_ordering
Given a class defining one or more rich comparison ordering methods, this class decorator supplies the rest. This simplifies the effort involved in specifying all the possible rich comparison operations:

The class must define one of __lt__(), __le__(), __gt__(), or __ge__(). In addition, the class should supply an __eq__() method.

> While this decorator makes it easy to create well-behaved totally ordered types, it does come at the cost of slower execution and more complex stack traces for the derived comparison methods. If performance benchmarking indicates this is a bottleneck for a given application, implementing all six rich comparison methods instead is likely to provide an easy speed boost.

> This decorator makes no attempt to override methods that have been declared in the class or its superclasses. Meaning that if a superclass defines a comparison operator, total_ordering will not implement it again, even if the original method is abstract.

## functools.reduce(function, iterable[, initializer])

Apply function of two arguments cumulatively to the items of iterable, from left to right to reduce the iterable to a single value. For example, `reduce(lambda x, y: x+y, [1, 2, 3, 4, 5]) `calculates `((((1+2)+3)+4)+5)`. The left argument, x, is the accumulated value and the right argument, y, is the update value from the iterable. If the optional initializer is present, it is placed before the items of the iterable in the calculation, and serves as a default when the iterable is empty. If initializer is not given and iterable contains only one item, the first item is returned.

## functools.partial(func, /, *args, **keywords)

## class functools.partialmethod(func, /, *args, **keywords)

https://docs.python.org/3/library/functools.html