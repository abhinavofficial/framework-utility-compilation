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

If not managed, there could be multiple people asking for changes within the same module. It may cause fragility. A piece of code change may break completely unrelated code function, causing management panic. Management can thereby impose restriction on code change, leading to forced rigidity. 

Solution:
1. Split responsibility
2. Facade pattern: A partial solution. Facade class do not do anything - it delegates the implementation to other classes. This separate implementation and preserves interface. Since the implementation is the one that typically cause this problem and not interface, this solution can solve SRP issues.


## Open Closed Principle
## Liskov Substitute Principle
## Interface Segregation Principle
## Dependency Inversion Principle
* Functional Design:
    * All functional programming has some kind of polymorphism. Polymorphismic 
    * Multi method