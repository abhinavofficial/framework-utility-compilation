# Creational Patterns
Object creation has five design patterns mentioned.
* Factory Method
* Abstract Factory
* Prototype
* Builder
* Singleton

## Factory Method
A Factory Pattern or Factory Method Pattern says that just **define an interface or abstract class for creating an object but let the subclasses decide which class to instantiate**. In other words, subclasses are responsible to create the instance of the class.

In general, we have **product** (defines the factory interface of objects), **concreteProduct** (implements the product interface), **creator** (declares the factory) and **concreteCreator** (overrides the creator and returns an instance of concreteProduct)

It is aligned with SOLID (Single Responsibility Principle, Liskov Substitution Principle, )  - TODO Does it follow O, I and D

## Abstract Factory
An Abstract Factory Pattern or Abstract Factory Method Pattern says that just **provide an interface for creating families of related or dependent objects without specifying their concrete classes**. In other words, object instantiation is coded in methods exposed by the factory interface.

Choose the abstract factory pattern when:
* A system has to be independent of how its products are created, and represented.
* A system needs to provide a class library of products, and to reveal just their interfaces, not their implementations.
* A system should be able to support multiple families of products.

One of the benefits of Abstract factory patterns is:
* Isolates clients from concrete implementation classes to manage things like Separation of Concerns, Object Creation and Object Lifetime Management.
* Makes exchanging product families easy, since a particular concrete factory can support complete family of products
* Enforces the use of products only from one family.

Pitfall of using this method:
* Supporting new kinds of products requires changing AbstractFactory interface which implies that all of its derived concrete classes also must change
* Incompleteness of GoF version: GoF Abstract Factory only covers object creation, not object disposal.

Usage is most common where we deal with interoperability.

It is aligned with SOLID (Single Responsibility Principle, Open/Close Principle, Liskov Substitution Principle) - TODO Does it follow I and D

## Prototype
Prototype lets you copy existing objects without making your code dependent on their classes.

[Wikipedia Link](https://en.wikipedia.org/wiki/Prototype_pattern)

## Builder

## Singleton

## Factory and Abstract Factory - Differences
Common thing is that
* All the factories encapsulate object creation.
* Factory is the simplest way to decouple clients from implementation classes.
* All the factories provide loose coupling thereby making the design flexible.

However, while factory method depends on inheritance (child class instantiates), abstract factory depends on object composition (interface instantiates)

Abstract Factory is also called factory of factories.