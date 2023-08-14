# Numbers in Python
Number data types store numeric values. They are immutable data types. This means, changing the value of a `number` data type results in a newly allocated object.

* Number objects are created when you assign a value to them.
* You can also delete the reference to a number object by using the `del` statement.
* Python supports different numerical types − 
  * int (signed integers) − They are often called just integers or ints. They are positive or negative whole numbers with no decimal point. Integers in Python 3 are of unlimited size. Python 2 has two integer types - int and long. There is no 'long integer' in Python 3 anymore. 
  * float (floating point real values) − Also called floats, they represent real numbers and are written with a decimal point dividing the integer and the fractional parts. Floats may also be in scientific notation, with E or e indicating the power of 10 (2.5e2 = 2.5 x 102 = 250). 
  * complex (complex numbers) − are of the form a + bJ, where a and b are floats and J (or j) represents the square root of -1 (which is an imaginary number). The real part of the number is a, and the imaginary part is b. Complex numbers are not used much in Python programming.
* It is possible to represent an integer in hexa-decimal or octal form.

## Number Type Conversion

Python converts numbers internally in an expression containing mixed types to a common type for evaluation. Sometimes, you need to coerce a number explicitly from one type to another to satisfy the requirements of an operator or function parameter.

* Type int(x) to convert x to a plain integer. 
* Type long(x) to convert x to a long integer. 
* Type float(x) to convert x to a floating-point number. 
* Type complex(x) to convert x to a complex number with real part x and imaginary part zero. 
* Type complex(x, y) to convert x and y to a complex number with real part x and imaginary part y. x and y are numeric expressions

| Number | function                          | Description                                                                                                                                          | 
|--------|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1      | abs(x)                            | The absolute value of x: the (positive) distance between x and zero.                                                                                 |
| 2      | ceil(x)                           | The ceiling of x: the smallest integer not less than x.                                                                                              |
| 3      | cmp(x, y)                         | -1 if x < y, 0 if x == y, or 1 if x > y. Deprecated in Python 3. Instead use return (x>y)-(x<y).                                                     |
| 4      | exp(x)                            | The exponential of x: ex                                                                                                                             |
| 5      | fabs(x)                           | The absolute value of x.                                                                                                                             |
| 6      | floor(x)                          | The floor of x: the largest integer not greater than x.                                                                                              |
| 7      | log(x)                            | The natural logarithm of x, for x > 0.                                                                                                               |
| 8      | log10(x)                          | The base-10 logarithm of x for x > 0.                                                                                                                |
| 9      | max(x1, x2,...)                   | The largest of its arguments: the value closest to positive infinity                                                                                 |
| 10     | min(x1, x2,...)                   | The smallest of its arguments: the value closest to negative infinity.                                                                               |
| 11     | modf(x)                           | The fractional & integer parts of x in a two-item tuple. Both parts have same sign as x. Integer part is returned as a float.                        |
| 12     | pow(x, y)                         | The value of x**y.                                                                                                                                   |
| 13     | round(x [,n])                     | x rounded to n digits from the decimal point. Python rounds away from zero as a tie-breaker: round(0.5) is 1.0 and round(-0.5) is -1.0.              |
| 14     | sqrt(x)                           | The square root of x for x > 0.                                                                                                                      |
| 15     | choice(seq)                       | A random item from a list, tuple, or string.                                                                                                         |
| 16     | randrange ([start,] stop [,step]) | A randomly selected element from range(start, stop, step).                                                                                           |
| 17     | random()                          | A random float r, such that 0 is less than or equal to r and r is less than 1                                                                        |
| 18     | seed([x])                         | Sets the integer starting value used in generating random numbers. Call this function before calling any other random module function. Returns None. |
| 19     | shuffle(lst)                      | Randomizes the items of a list in place. Returns None.                                                                                               |
| 20     | uniform(x, y)                     | A random float r, such that x is less than or equal to r and r is less than y.                                                                       |

## Mathematical constants
* pi: The mathematical constant pi.
* e: The mathematical constant e.
