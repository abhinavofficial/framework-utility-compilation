# Structural Patterns
These concern class and object composition. They use inheritance to compose interfaces and define ways to compose objects to obtain new functionality. Sometimes, it flexibly composes objects to realize new functionality such as composite.

## Adapter 
It allows classes with incompatible interfaces to work together by wrapping its own interface around that of an already existing class. There are two kinds of adapters - class and object adapters. Class adapter essentially provides a compile-time binding of the adapter object to the adaptee. Object adapter on the other hand provides a run-time binding.

## Bridge 
It decouples an abstraction from its implementation so that the two can vary independently.

## Composite
It composes zero-or-more similar objects so that they can be manipulated as one object. It allows representing the whole-part hierarchies where elements share many properties and helps achieve use cases where:
* Clients do not want to distinguish whether they operate on a compound object or on an atomic object.
* The hierarchical structure of the compound object should always be preserved
* Clients sometimes want to re-organize the hierarchy
* Over time, it may be necessary to extend the hierarchy with new element types, yet existing code and configuration should remain unaffected.

Based on above, the composite represent compound objects as tree structures:
* The **component** is an abstract class or an interface for object in the composition
* **Leaf** element represents the smallest objects in the composition
* A **composite** element is the concrete implementation for composed objects.

Now, lets see how objects collaborate in this design method. A client invokes an operation on a composite structure. If the object that receives the request is
* a leaf, the operation is executed on the leaf.
* a composite, the composite invokes the operation on all its children or can select one of its children to execute the operation.

Benefits
* Transparency: Clients are shielded from the object hierarchy.
* Extensibility: New leafs / composites are easy to add.
* Genericity: Arbitrary object hierarchy can be built

Liabilities
* Limited applicability: Can make design overly general. It is hard to restrict the component of a composite, can pose a problem when you want only certain type of components to be in the composite.
* Difficult to debug due to its recursive nature of build.

However, it finds its usage in
* topological trees of all kind: physical warehouse structures, telecom and electrical networks, railroad networks, etc.
* GUI frameworks: Smalltalk, ET++, Interviews, others use composite arrangements to represent views.
* Compilers: Parse trees are often represented as composite hierarchies.

Please note some implementation issues
* References from child components to their parent has to be maintained to simplify the navigation of a composite structure
* Declaring the child access operations. Where should the child access operations be declared? It is recommended to have it in a separate class.
* Should component implement a list of components.
* Child ordering
* Caching to improve performance.

It follows SRP, OCP, ISP and DIP of the SOLID but not LSP.

## Decorator 
It dynamically adds/overrides behaviour in an existing method of an object.

## Facade
It provides a simplified interface to a large body of code, sometime an entire subsystem.

## Flyweight 
It reduces the cost of creating and manipulating a large number of similar objects. It efficiently and consistently share objects.

## Proxy
It provides a placeholder for another object to control access, reduce cost, and reduce complexity.