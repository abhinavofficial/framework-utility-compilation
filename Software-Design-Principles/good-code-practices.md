# Good Code Design
Write code with good design with abstraction can be considered as beautiful code and with it, as ugly code.

## Code Smell
Broadly, three smells -

[PEP 8 - Style Guide for Python Code](https://peps.python.org/pep-0008/)

### Syntactics smells
* Code must be easy to read
* Assumptions must be explicit in the code.
* Name of variable or function must reflect how it would be used. It should not too short and too long. Follow the naming as implied.
* Comment should reflect what the code is trying to do.
* Function should not take too many input parameter or return too many output parameters.

### Semantic smells
Program Abstraction is the key. Two simple abstractions (not exhaustive list) are - 
* Functional
  * each function should do one well-defined task (around 10 lines of code)
  * poorly designed functional abstraction could result in long methods, duplicated functions and undesirable side effects (e.g. mutation, any change in one part of function impacting other parts - especially variable scoping).
  * poorly logical abstraction is when control flow and logic is complicated.
  * sequential abstraction (a set of tasks grouped into a function executed one after the other) can help when tasks are dependent, sequential and related. It is desirable but less desirable than functional cohesion.
  * Unrelated logic in a single function/class (cohesion). One way to detect bad cohesion is to understand what is the data being consumed by each of the methods. If they are very unrelated, then it indicates this problem.
  * Over-complexity at line, function and class level. E.g. Too many nested loops and conditional statement in a single line.
  
* class/object - we need to have complete interface
  * Class is a cohesive entity.
  * A good abstraction for a class requires that the interface is complete. Reusable classes such as class libraries are examples of good abstraction.
  * A single class should not have the responsibility to manage different object types. A fat / large class should be refactored (maybe wrapped with a class)
  * Inheriting a smaller method from base class and overriding it in the derived class is preferred than inheriting a large method and override it in the derived class. This is called **Narrow Inheritance**
  * Introducing an abstraction that is not actually needed or if a class has large number of inherited classes - these are signals of poor abstraction.

### Deep smells
These arise from issues like inefficient codes, poor memory management, poor concurrency management, poor race condition handling, etc. It is little difficult to detect - it may require specialized tools to detect like stress testing, etc.
Some of these are also programming language specific. For example, we can probably use ```enumerate(list)``` instead of ```range(len(list))``` or using a ```loop``` instead using ```comprehension```