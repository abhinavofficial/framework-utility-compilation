# C and C++

| C                                 | C++                   |
|-----------------------------------|-----------------------|
| Procedural                        | C with classes        |
| Very efficient - speed and memory | Less efficient than C |
| Easily create bad designs         | Supports OOP          |

C++ tried improvement to dispatch the polymorphic method using **VTABLE**. VTABLE mechanism mitigates the efficiency concern -
* Compiler creates VTABLE for each class.
* Places each virtual function at fixed offset.
* Install VPTR to reference VTABLE
  Now when late binding is called for, the code follows VPTR at run-time.

In case of multiple inheritance (allowed by C++ but not Java), when you make a call to one of the base class methods, then through indirection the base class could be calling its sibling class methods connected by multiple inheritance. The context can also decide which method to invoke. This is also called message passing  between the sibling.