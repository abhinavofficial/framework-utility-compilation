# Good Code Design
Write code with good design with abstraction can be considered as beautiful code and with it, as ugly code.

## Context
Code maintenance takes about 2/3 of the entire code lifecycle. This cost is high because the code is messy, which makes it hard to:
* Understand existing code
* Find and fix bugs
* Add new features
* Adapt to new technology

Let's understand some of the symptoms of the bad design.
* **Rigidity**: The system is hard to change because every change forces many other changes to other parts of the system. [Remember the line: It was more complex than I thought]
* **Fragility**: Hidden dependencies make systems easy to break. The consequences of change are not localized.
* **Immobility**: It is hard to disentangle the system into components that can be reused in another systems.
* **Viscosity**: Tools, environment, and design impede progress. It is slower to do the right thing than the wrong thing.
* **Needless complexity, repetition**: 
* **Opacity**: Hard to understand.

Below the bad designs are code smells.
* Duplication (DRY): Accidental duplication - which can evolve differently is fine.
* Long method
* Big Class
* Feature envy
* Useless comment
* Poor name
* Inappropriate intimacy
* Shotgun surgery
* Switch
* God Class
* Primitive Obsession
* Refused Bequest


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

## Remedies for bad designs
Primarily using dependencies Management: Organize the system to keep it untangled. The way to do this is to separate concerns - build a firewall across components across which changes cannot propagate. This is what can be achieved by class design principles called [SOLID](design-principles/solid-design-principle.md) principles. There are other set of 6 principles called the [component](design-principles/component-principles.md) and [architecture](design-principles/architecture-principles.md) principles.

> Class is used here symbolically. There are many programming languages which does not use the keyword class, but has conception of data elements and functions which forms a cohesive unit.

### Symptoms of a good design
* Reveal Intent
  * Provides insights into requirements analysis
* Adaptable
  * Tolerates evolution of business needs
  * Accepts new technologies without undue cost - Cost of the change is proportional to scope of the change

Code, Design and Architecture are always in continuum. If one is bad, the other cannot be okay, at least for a sustainable period of time.

## Professionalism
Doing right requires some bit of professionalism.

* Check the code in cleaner than you checked out every single time. You have to craft it, and clean it.
* The only way to go fast is to go well. Rushing does not help. This is our professional responsibility.


## References
* Martin Fowler - Refactoring
* Tools: CRAP: High cyclomatic complexity of a function against the code coverage.
* Testing suiting is critical.


https://www.youtube.com/watch?v=7EmboKQH8lM&list=PLmmYSbUCWJ4x1GO839azG_BBw8rkh-zOj