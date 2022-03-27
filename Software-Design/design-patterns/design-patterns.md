# Design Patterns

A pattern describes a **problem** which occurs over and over again in our environment, and then describes the core of the **solution** to the problem, in such a way that you can reuse this solution a million times over without ever doing it the same way twice, as described by Christopher Alexander. It is not domain specific.

A design pattern is a representation of **communicating objects and classes** that are customized to solve a general design problem in a particular context, as described by GoF, Design Patterns.

To come up with a design Pattern, below are the three steps involved:
* An elegant idea solving a design problem
* Abstracts the core of the design solution (into an interacting objects and classes)
* Can be reused in various contexts

## Format of a design pattern
* Pattern name and classification
* Intent
  * What does the pattern do?
  * What particular design issue or problem does it address?
* Also Known As
  * Other names of the pattern
* Motivation
  * How the class and object structures solve the problem
* Applicability
  * Situations in which the pattern can be applied  (and not applied)
* Structure
  * Graphical representation of the classes in the pattern, with interactions
* Participants
  * The classes and/or objects participating in the pattern
  * Their responsibilities
* Collaborations
  * How participants collaborate to carry out responsibilities
* Consequences
  * How the pattern supports its objectives
  * Trade-offs and result
* Implementation
  * Hints and pitfalls
  * Language specific issues
  * Illustrate how one could implement the pattern
* Sample Code
  * Code fragments
* Known Uses
  * Example of pattern use in a real system
* Related Patterns
  * Closely related patterns

## Key to design patterns
The key to design pattern is interacting classes/objects.

What is varying would be abstract out of the class from fixed behavior. Have concrete classes to inherit the varying behavior abstract class.

## Pattern Categories
Based on purpose (what does it do), there are three categories
* [Creational Patterns](creational-patterns.md)
* [Structural Patterns](structural-patterns.md)
* [Behavioral Patterns](behavioral-patterns.md)

## How to really use pattern

### Motivation
To understand how to apply design pattern in real life implementation in an object-oriented programming design.

### Identification Classes and Responsibilities
Once the problem statement is defined, pick up all the nouns - they are kind of serve as entities / classes / objects. Define the responsibilities for these entities. While identifying responsibilities, do not worry about interactions.

### Class Groups and Interactions
Once classes and responsibilities are identified, now
* Identify all pairs of classes whose objects interact
* Use adjectives

To identify class groups, create collection of interacting classes with varying strength (count of interacting classes). For example, Group of strength 1 is the class itself, Group of strength 2 are the ones where only two classes are selected based on interaction, and so on (of course, the max strength will be the total number of classes).

Now, in each kind of these groups, we focus on what kind of interactions exist and its characteristics.

### Identifying Design Patterns
Now compare the abstractions with known design patterns and get the possibilities. When multiple patterns are derived which affects the same set of classes and cannot co-exist, we can derive new rough designs using them.
Then do the trade-off analysis on rough designs to select the final design.

## Design versus Performance

Design for functionality and maintenance is more important than design for performance. Once you have the design right, then we can focus on how to improve on performance, if needed.

Design Goals to achieve coupling, cohesion and patterns:
* Simplify the interface
* Increase flexibility 

Performance Goals:
* Reduce processing time
* Shrink memory footprint

Sometime, performance is so critical that it has to become a functional requirement and hence, design goals should include this.

From a language perspective, there are two -
* Procedural Programming
  * Scattered state and behavior
  * Difficult to main programs
* Object-Oriented Programming (OOP)
  * Encapsulated state and behavior
  * Efficiently maintainable programs
