# Behavioral Patterns
Behavior pattens describe both objects, communication and assignment of responsibilities between them.
There are two mechanisms -
* Class patterns - Inheritance to distribute behavior
* Object patterns - Composition to delegate behavior, example Chain of Responsibility pattern for cooperation between peers or Strategy pattern for delegating requests.

Behavioral patterns can be viewed in
* Abstract what varies
  * Iterator
  * Strategy
  * State
* Decouple senders from receivers
  * Chain of Responsibility
  * Command
  * Observer
* Double dispatch
  * Visitor

TODO fit others

## Chain of responsibility
It delegates commands to a chain of processing objects. 

Every class/object having the responsibility of task(s) can also pass the request to the other class/object handler.

A slight deviation of this is leader-follower pattern where one object responds to client's requests.

Please ensure to have a default handler in the CoR.

## Command
It creates objects that encapsulate actions and parameters.

## Interpreter
It implements a specialized language.

## Iterator
It accesses the elements of an object sequentially without exposing its underlying representation.

Iterator class is an abstract class, the concrete implementation of which is available in the child class to effectively allow access to elements sequentially to solve the problem.

## Memento
It provides the ability to restore an object to its previous state (undo).

## Observer
It is a publish/subscribe pattern, which allows a number of observer objects to see an event.

### Problem it addresses
When the internal state of an object changes, other objects that are dependent on it need to be informed. How can this be done such that
* Information provider is only loosely coupled with information consumers
* Information consumers that depend upon the information provider should not be known beforehand.
* Information provider should not know who are information consumers.

### Structure
* Implement a change propagation mechanism between the information provider and the information consumers (the observers)
* The provider maintains a registry of observers and notifies all the registered observers about changes to its state.
* An observer declares an update function to be called by the provider's change propagation mechanism.
* Concrete observers implement the update function in a system-specific manner.

### Collaboration
* The observers register with the provider's change propagation mechanism.
* A client modifies the provider's data.
* The provider starts its change propagation mechanism to call the update function of all registered observers.
* The observers retrieve the changed data from the provider and update themselves.

### Pros and Cons
Pros
* Defined handling of dependencies between otherwise strongly coupled objects.
* Support for dynamic configuration of a provider with observers.
* Adding a new observer does not affect the provider or its change propagation mechanism

Cons
* Unnecessary updates may be received
* Cascade of updates
* Indirection - new data value is not directly received - only notification of change is received.

It is aligned with SRP, OCP, LSP, ISP, DIP of SOLID

## Mediator
It allows loose coupling between classes by being the only class that has detailed knowledge of their methods.
On a very high level, this is really a small variant of observer pattern. Almost like being a change manager (mediator acting a many-to-many relation between providers and consumers) which helps encapsulate complex dependency relationships between subjects and observers and eliminates the need for provider to maintain references to their observers.


## State
It allows an object to alter its behavior when its internal state changes.

## Strategy
It allows one of a family of algorithms to be selected on-the-fly at runtime.

## Template
It method defines the skeleton of an algorithm as an abstract class, allowing its subclasses to provide concrete behavior.

## Visitor
It separates an algorithm from an object structure by moving the hierarchy of methods into one object. So, in this case, algorithm or operation hierarchy can call the object hierarchy and vice versa making this a double dispatch mechanism or visitor method.

```
When we see word - **accept** or **visit**, it is 
```

Traits as implementation.