# Inheritance and Composition

## Composition
We can look at composition as **has-a relationship** or **is-part-of** relationship.
In a has-a relationship, two entity are independent, like classroom and teacher. In an is-part-of relationship, there is entity which owns the other entity, like car and wheel.
If we delete a owning object in an is-part-of relationship, the owned object is deleted as well. This is not the same in has-a relationship.
Has-a relationship is a run-time dependency. In Is-a relationship, the relation is established at compile-time.

## Inheritance Vs Composition
Inheritance can be thought of **is-a** relationship. Example, Engineer is-a person. The relationship get established at compile time. Contrast that with Composition's has-a relationship.