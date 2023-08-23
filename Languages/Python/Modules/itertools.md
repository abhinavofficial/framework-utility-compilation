# itertools - Functions creating iterators for efficient looping
The module standardizes a core set of fast, memory efficient tools that are useful by themselves or in combination. Together, they form an “iterator algebra” making it possible to construct specialized tools succinctly and efficiently in pure Python.

For instance, SML provides a tabulation tool: tabulate(f) which produces a sequence f(0), f(1), .... The same effect can be achieved in Python by combining map() and count() to form map(f, count()).

These tools and their built-in counterparts also work well with the high-speed functions in the operator module. For example, the multiplication operator can be mapped across two vectors to form an efficient dot-product: sum(starmap(operator.mul, zip(vec1, vec2, strict=True))).

## Infinite iterators

| Iterator | Arguments     | Results                                        | Example                               |
|----------|---------------|------------------------------------------------|---------------------------------------|
| count()  | start, [step] | start, start+step, start+2*step, …             | count(10) --> 10 11 12 13 14 ...      |
| cycle()  | p             | p0, p1, … plast, p0, p1, …                     | cycle('ABCD') --> A B C D A B C D ... |
| repeat() | elem [,n]     | elem, elem, elem, … endlessly or up to n times | repeat(10, 3) --> 10 10 10            |

```python
import itertools as it
evens = it.count(step=2)
list(next(evens) for _ in range(5))
```


## Iterators terminating on the shortest input sequence

| Iterator              | Arguments                   | Results                                    | Example                                                  |
|-----------------------|-----------------------------|--------------------------------------------|----------------------------------------------------------|
| accumulate()          | p [,func]                   | p0, p0+p1, p0+p1+p2, …                     | accumulate([1,2,3,4,5]) --> 1 3 6 10 15                  |
| chain()               | p, q, ...                   | p0, p1, … plast, q0, q1, …                 | chain('ABC', 'DEF') --> A B C D E F ...                  |
| chain.from_iterable() | iterable                    | p0, p1, … plast, q0, q1, …                 | chain.from_iterable('ABC', 'DEF') --> A B C D E F ...    |
| compress()            | data, selectors             | (d[0] if s[0]), (d[1] if s[1]), …          | compress('ABCDEF', [1,0,1,0,1,1]) --> A C E F            |
| dropwhile()           | pred, seq                   | seq[n], seq[n+1], starting when pred fails | dropwhile(lambda x: x<5, [1,4,6,4,1]) --> 6 4 1          |
| filterfalse()         | pred, seq                   | elements of seq where pred(elem) is false  | filterfalse(lambda x: x%2, range(10)) --> 0 2 4 6 8      |
| groupby()             | iterable[, key]             | sub-iterators grouped by value of key(v)   |                                                          |
| islice()              | seq, [start,] stop [, step] | elements from seq[start:stop:step]         | islice('ABCDEFG', 2, None) --> C D E F G                 |
| pairwise()            | iterable                    | (p[0], p[1]), (p[1], p[2])                 | pairwise('ABCDEFG') --> AB BC CD DE EF FG                |
| starmap()             | func, seq                   | func(*seq[0]), func(*seq[1]), …            | starmap(pow, [(2,5), (3,2), (10,3)]) --> 32 9 1000       |
| takewhile()           | pred, seq                   | seq[0], seq[1], until pred fails           | takewhile(lambda x: x<5, [1,4,6,4,1]) --> 1 4            |
| tee()                 | it, n                       | it1, it2, … itn splits one iterator into n |                                                          |
| zip_longest()         | p, q, ...                   | (p[0], q[0]), (p[1], q[1]), …              | zip_longest('ABCD', 'xy', fillvalue='-') --> Ax By C- D- |


## Combinatoric iterators

| Iterator                        | Arguments          | Results                                                       | Example                                                                       |
|---------------------------------|--------------------|---------------------------------------------------------------|-------------------------------------------------------------------------------|
| product()                       | p, q, … [repeat=1] | cartesian product, equivalent to a nested for-loop            | product('ABCD', repeat=2) --> AA AB AC AD BA BB BC BD CA CB CC CD DA DB DC DD |
| permutations()                  | p[, r]             | r-length tuples, all possible orderings, no repeated elements | permutations('ABCD', 2) -->  AB AC AD BA BC BD CA CB CD DA DB DC              |
| combinations()                  | p, r               | r-length tuples, in sorted order, no repeated elements        | combinations('ABCD', 2) --> AB AC AD BC BD CD                                 |
| combinations_with_replacement() | p, r               | r-length tuples, in sorted order, with repeated elements      | combinations_with_replacement('ABCD', 2) --> AA AB AC AD BB BC BD CC CD DD    |

## Itertools Recipes
This section shows recipes for creating an extended toolset using the existing itertools as building blocks.

The primary purpose of the itertools recipes is educational. The recipes show various ways of thinking about individual tools — for example, that chain.from_iterable is related to the concept of flattening. The recipes also give ideas about ways that the tools can be combined — for example, how compress() and range() can work together. The recipes also show patterns for using itertools with the operator and collections modules as well as with the built-in itertools such as map(), filter(), reversed(), and enumerate().

A secondary purpose of the recipes is to serve as an incubator. The accumulate(), compress(), and pairwise() itertools started out as recipes. Currently, the iter_index() recipe is being tested to see whether it proves its worth.
```python
python -m pip install more-itertools
```


https://docs.python.org/3/library/itertools.html#itertools-recipes