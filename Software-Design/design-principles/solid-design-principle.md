# SOLID design principle

It was first coined by Robert C. Martin in his book - Agile Software Development: Principles, Patterns, and Practices.

Other 
* Using comp.object debates - Google Archive (mid 90s)
* NCARB Project was referenced where reusable framework
* Some principles came from James O. Coplien, Barbara Liskov and Bertrand Meyer
* Designing Object-Oriented C++ Applications using the Booch Method

## Single Responsibility Principle
Named by Bertrand Meyer

* A class / module should have one and only reason to change.
* An actor, by definition is the source of the change (who asked / asks for the change).
* Module should be responsible for one and only one actor.

If not managed, there could be multiple people asking for changes within the same module. It may cause fragility. A piece of code change may break completely unrelated code function, causing management panic. Management can thereby impose restriction on code change, leading to forced rigidity. There are other issues, like
* Multiple people in the same module for different reasons leading to SCM issues

Solution:
1. Split responsibility
2. Facade pattern: A partial solution. Facade class do not do anything - it delegates the implementation to other classes. This separate implementation and preserves interface. Since the implementation is the one that typically cause this problem and not interface, this solution can solve SRP issues.

**Conway's law**: Any organization that designs a system (defined broadly) will produce a design whose structure is a copy of the organization's communication structure. SRP system says the architecture of the software should look the **organization** of the users (not the programmers). Hope you can note the similarity.


## Open Closed Principle
Coined by Bertrand Meyer in 1988 in his book - Object-Oriented Software Construction.

A software module should be open for extension, which means we are able to change or extend the behavior of a system. It should also be closed for modification, which means we should not have to modify a module for extending the functionality.

So, simply speaking - You can add a new feature by adding new code without changing the existing code.

Often when this principle is violated, the intent of module is not appropriately exposed.

## Liskov Substitute Principle
## Interface Segregation Principle
## Dependency Inversion Principle
* Functional Design:
    * All functional programming has some kind of polymorphism. Polymorphismic 
    * Multi method


https://betterprogramming.pub/5-problems-faced-when-using-solid-design-principles-and-how-to-fix-them-df6dbf3699fb

https://www.freecodecamp.org/news/beyond-unit-tests-an-intro-to-property-and-law-testing-in-scala-dd6a15898a19/

https://marketsplash.com/tutorials/scala/scala-value-types/

https://www.klazz.net/pub/icstw2014-kyatoh.pdf

https://morioh.com/a/8f691143fab9/testing-with-scala-library-landscape

https://medium.com/@saschagrunert/demystifying-containers-part-i-kernel-space-2c53d6979504
