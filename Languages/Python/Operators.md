# Operator in Python
Operators in general are used to perform operations on values and variables. These are standard symbols used for the purpose of logical and arithmetic operations. 

Two keywords used in this context:
* OPERATORS: These are the special symbols. Eg- + , * , /, etc.
* OPERAND: It is the value on which the operator is applied.

## Arithmetic Operators in Python

| Operator | 	Description                                                                    | Syntax  |
|----------|---------------------------------------------------------------------------------|---------|
| +        | 	Addition: adds two operands                                                    | x + y   |
| –        | 	Subtraction: subtracts two operands                                            | x – y   |
| *        | 	Multiplication: multiplies two operands                                        | x * y   |
| /        | 	Division (float): divides the first operand by the second                      | x / y   |
| //       | 	Division (floor): divides the first operand by the second                      | x // y  |
| %        | Modulus: returns the remainder when the first operand is divided by the second  | x % y   |
| **       | 	Power: Returns first raised to power second                                    | x ** y  |

> In Python 3.x the result of division is a floating-point while in Python 2.x division of 2 integers was an integer. To obtain an integer result in Python 3.x floored (// integer) is used. Please note however, if any of the numbers is float, it returns output in float.
> Precedence of Arithmetic Operators in Python is as follows (same as BODMAS, but there is no of in Python): P – Parentheses -> E – Exponentiation -> D – Division -> M – Multiplication (Multiplication and division have the same precedence) -> A – Addition (Addition and subtraction have the same precedence) -> S – Subtraction

## Comparison Operators in Python

| Operator | 	Description                                                                             | Syntax |
|----------|------------------------------------------------------------------------------------------|--------|
| >        | 	Greater than: True if the left operand is greater than the right                        | x > y  |
| <        | 	Less than: True if the left operand is less than the right                              | x < y  |
| ==       | 	Equal to: True if both operands are equal                                               | x == y |
| !=       | 	Not equal to – True if operands are not equal                                           | x != y |
| >=       | 	Greater than or equal to True if the left operand is greater than or equal to the right | x >= y |
| <=       | Less than or equal to True if the left operand is less than or equal to the right        | x <= y |

> In python, the comparison operators have lower precedence than the arithmetic operators. All the operators within comparison operators have same precedence order.
> Both “is” and “==” are used for object comparison in Python. The operator “==” compares values of two objects, while “is” checks if two objects are same (In other words two references to same object).

## Logical Operators in Python

| Operator | 	Description                                        | Syntax  |
|----------|-----------------------------------------------------|---------|
| and      | 	Logical AND: True if both the operands are true    | x and y |
| or       | 	Logical OR: True if either of the operands is true | x or y  |
| not      | 	Logical NOT: True if the operand is false          | not x   |

> The precedence of Logical Operators in python is as follows: Logical not -> logical and -> logical or

## Bitwise Operators in Python

Python Bitwise operators act on bits and perform bit-by-bit operations. These are used to operate on binary numbers.

| Operator | 	Description         | Syntax |
|----------|----------------------|--------|
| &        | 	Bitwise AND         | x & y  |
| \|       | 	Bitwise OR          | x \| y |
| ~        | 	Bitwise NOT         | ~x     |
| ^        | 	Bitwise XOR         | x ^ y  |
| >>       | 	Bitwise right shift | x>>    |
| <<       | Bitwise left shift   | x<<    |

> The precedence of Bitwise Operators in python is as follows: Bitwise NOT -> Bitwise Shift -> Bitwise AND -> Bitwise XOR -> Bitwise OR

## Assignment Operators in Python

| Operator | 	Description                                                                                                | 	Syntax                      |
|----------|-------------------------------------------------------------------------------------------------------------|------------------------------|
| =	       | Assign the value of the right side of the expression to the left side operand 	                             | x = y + z                    |
| +=	      | Add AND: Add right-side operand with left-side operand and then assign to left operand	                     | a+=b same as    a=a+b        |
| -=	      | Subtract AND: Subtract right operand from left operand and then assign to left operand	                     | a-=b same as    a=a-b        |
| *=	      | Multiply AND: Multiply right operand with left operand and then assign to left operand	                     | a*=b same as    a=a*b        |
| /=	      | Divide AND: Divide left operand with right operand and then assign to left operand	                         | a/=b  same as   a=a          |
| %=	      | Modulus AND: Takes modulus using left and right operands and assign the result to left operand	             | a%=b  same as   a=a%b        |
| //=	     | Divide(floor) AND: Divide left operand with right operand and then assign the value(floor) to left operand	 | a//=b same as    a=a//b      |
| **=	     | Exponent AND: Calculate exponent(raise power) value using operands and assign value to left operand	        | a**=b same as    a=a**b      |
| &=	      | Performs Bitwise AND on operands and assign value to left operand	                                          | a&=b same as    a=a&b        |
| \| =	    | Performs Bitwise OR on operands and assign value to left operand	                                           | a\|=b  same as   a=a\|b      |
| ^=	      | Performs Bitwise xOR on operands and assign value to left operand	                                          | a^=b same as    a=a^b        |
| >>=	     | Performs Bitwise right shift on operands and assign value to left operand	                                  | a>>=b  same as   a=a>>b      |
| <<=	     | Performs Bitwise left shift on operands and assign value to left operand	                                   | a <<= b  same as   a= a << b |


## Identity Operators in Python

In Python, `is` and `is not` are the identity operators both are used to check if two values are located on the same part of the memory. Two variables that are equal do not imply that they are identical.
```text
is          True if the operands are identical
is not      True if the operands are not identical 
```

## Membership Operators in Python

In Python, `in` and `not in` are the membership operators that are used to test whether a value or variable is in a sequence.

```text
in            True if value is found in the sequence
not in        True if value is not found in the sequence
```

## Ternary Operator in Python

in Python, Ternary operators also known as conditional expressions are operators that evaluate something based on a condition being true or false. It was added to Python in version 2.5.  It simply allows testing a condition in a single line replacing the multiline if-else making the code compact.
```python
# Syntax :  [on_true] if [expression] else [on_false]
a, b = 10, 20
min = a if a < b else b
print(min)
```

## Precedence and Associativity of Operators in Python
In Python, Operator precedence and associativity determine the priorities of the operator.

| Precedence | Operators                                    | Description                                                 | Associativity |
|------------|----------------------------------------------|-------------------------------------------------------------|---------------|
| 1          | ()                                           | Parentheses                                                 | Left to right |
| 2          | x[index], x[index:index]                     | Subscription, slicing                                       | Left to right |
| 3          | await x                                      | Await expression                                            | N/A           |
| 4          | **                                           | Exponentiation                                              | Right to left |
| 5          | +x, -x, ~x                                   | Positive, negative, bitwise NOT                             | Right to left |
| 6          | *, @, /, //, %                               | Multiplication, matrix, division, floor division, remainder | Left to right |
| 7          | +, –                                         | Addition and subtraction                                    | Left to right |
| 8          | <<, >>                                       | Shifts                                                      | Left to right |
| 9          | &                                            | Bitwise AND                                                 | Left to right |
| 10         | ^                                            | Bitwise XOR                                                 | Left to right |
| 11         | \|                                           | Bitwise OR                                                  | Left to right |
| 12         | in, not in, is, is not, <, <=, >, >=, !=, == | Comparisons, membership tests, identity tests               | Left to Right |
| 13         | not x                                        | Boolean NOT                                                 | Right to left |
| 14         | and                                          | Boolean AND                                                 | Left to right |
| 15         | or                                           | Boolean OR                                                  | Left to right |
| 16         | if-else                                      | Conditional expression                                      | Right to left |
| 17         | lambda                                       | Lambda expression                                           | N/A           |
| 18         | :=                                           | Assignment expression (walrus operator)                     | Right to left |
