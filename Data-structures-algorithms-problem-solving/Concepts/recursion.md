# Recursion

## Pre-requisite

* Iteration and Loops:
Functions

Trees, Graphs, Sorts - Recursion

## Working of recursive function

Recursive function - function that calls itself. The inner function does the work based on the parameters passed and returns the value to its calling function. The call continues to call itself till the function hits its base condition - post this the function starts returning the value to its calling function.

Recursion occurs in the program memory in a stack form. This stack maintains the space of each of the parameters / variables / etc. for each function along with the position where the control was passed to the called function.
After the base condition is met, the stack keep going down and hitting the same point at which the recursive call was made and then finishes off the remainder of the code. Once that code is complete, this function pointer is removed and the return value gets passed to the calling function.

Since each layer of the function creates and maintains its own memory state, there is a risk of stack overflow if the recursion is not controlled.

## Recipe of recursion function

1. What should be the input to recursion function
2. What is the base condition? What should happen when the base condition is met
3. What is the work to do be done in the function.

### Ways to work with recursive problem

#### Basic Construct

* What is the simplest possible input? This often becomes the base condition?
* Play around with examples and visualize!
* Relate hard cases to simpler cases
* Generalize the pattern
* Write code by combining recursive pattern with the base case

#### Recursive Leap of Faith

* Assume simpler cases work out. So, if you have a question to find the sum all numbers starting at 0 up to 5, you can just add 5 to the sum of number starting at 0 up to 4 and assume that the later sum would work out itself.

[Pattern Search using Suffix Tree](https://www.geeksforgeeks.org/pattern-searching-using-suffix-tree/)
