# SOLID design principle

It was first coined by Robert C. Martin in his book - Agile Software Development: Principles, Patterns, and Practices.

Other 
* Using "comp.object" debates - Google Archive (mid 90s)
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

We should abstract things that are going to change. The issue is how do you know what will change? This is where Agile Software Development comes in handy, where we consult customers frequently and assess what is likely to change. DO NOT RUSH to abstract anything and everything. It would make the design too bulky.

"Pointer to implementation" or "pImpl" is a C++ programming technique that removes implementation details of a class from its object representation by placing them in a separate class, accessed through an opaque pointer.

Module that uses an interface which is the implemented in other classes - this is the most common way to create Open-Closed Principles.

Strategy Pattern is a very specific example of Open-Closed Principle.

## Liskov Substitute Principle
Created by Barabara Liskov in 1988.

Instances of a derive class must be usage through the interface of its base class without clients of the base class being able to tell the difference.

Subtypes must be substitutable for their base types in the programming context.

It sounds like basic polymorphism. However, it is slightly more than that. Anytime your subclass behaves differently than your base class, you are at risk of violating this principle.

Consider the venerable rectangle square problem. Even though Square is a rectangle, square class is not a rectangle class, and hence not substitutable. The classes are the representative of things in context. Representative rules states that representative of things do not share the relationship of the things they represent. [A Solution to the Square-Rectangle Problem Within the Framework of Object Morphology](https://aip.vse.cz/pdfs/aip/2016/01/03.pdf). 

How to guess of this violation?
* Anytime the derivative class does less than the base class, you may have problem. Example, your derived class override the base class function to do nothing, or throwing an exception.
* Derived class should also not do anything that the users of the base class do not expect.

Every violation of a Liskov Substitute principle inherently violates Open-Closed principles as well.

## Interface Segregation Principle
No code should be forced to depend on methods it does not use. ISP splits interfaces that are very large into smaller and more specific ones so that clients will only have to know about the methods that are of interest to them. Such shrunken interfaces are also called role interfaces. 

ISP is intended to keep a system decoupled and thus easier to refactor, change, and redeploy.

Languages like Ruby or Python or Clojure does not have interface as programming construct. But should still follow the principle - do not depend on method you do not use.

## Dependency Inversion Principle
It is the low level implementation of the high level intent of the other four principles.

In a procedural module: Main -> High -> Mid -> Low (Calls were same direction as the source code dependency)

OO changes the game where call is in the opposite direction of source code dependency.

One of the ways to achieve this is:
1. Abstract factory [goF]

## FAQ
1. Isn't it also possible to have one class or module for one actor but many reasons to change.

https://betterprogramming.pub/5-problems-faced-when-using-solid-design-principles-and-how-to-fix-them-df6dbf3699fb

https://www.freecodecamp.org/news/beyond-unit-tests-an-intro-to-property-and-law-testing-in-scala-dd6a15898a19/

https://marketsplash.com/tutorials/scala/scala-value-types/

https://www.klazz.net/pub/icstw2014-kyatoh.pdf

https://morioh.com/a/8f691143fab9/testing-with-scala-library-landscape

https://medium.com/@saschagrunert/demystifying-containers-part-i-kernel-space-2c53d6979504

* Functional Design:
  * All functional programming has some kind of polymorphism. Polymorphismic
  * Multi method

Suppose A and B are types, and T<U> denotes application of a type constructor T with type argument U. Within the type system of a programming language, a typing rule for a type constructor I is:
* **covariant** if it preserves the ordering of types (≤), which orders types from more specific to more generic: If A ≤ B, then I<A> ≤ I<B>
* **contravariant** if it reverses this ordering: If A ≤ B, then I<B> ≤ I<A> 
* **bi-variant** if both of these apply (i.e., if A ≤ B, then I<A> ≡ I<B>)
* **variant** if **covariant**, **contravariant** or **bivariant**
* **invariant** or **non-variant** if not variant.