# Feb updates

## Async Updates
The Async Foundations Working Group has been working to write a [shared async vision document](https://rust-lang.github.io/wg-async/vision.html)

> Keep watching wg-async-foundations work [project page](https://github.com/orgs/rust-lang/projects/2)

Useful command

find . -type d -name '4.1.68*' -exec rm -rf {}

## C++ RAII vs Rust OBRM

### C++ RAII
**R**esource **A**llocation **I**s **I**nitialization

**Resource** is something with a finite supply that requires management. Examples - Heap allocated memory, Network sockets, File Handles, Database handles, mutexes, etc.

1. Constructor and destructor should manage the allocation and deletion of resources.
2. Use of unique_ptr - one single owner
3. Use of shared_ptr - ownership is managed using reference counter. If reference counter becomes zero, resource is released.

```
void memory_example() {
    Car * car = new Car();
    function_that_can_throw();
    if(!should_continue()) return;
    delete car;
}

unique_ptr<T> t = make_unique<T>();
```

### Rust OBRM
Similar to RAII but instead of being a best-practice/pattern, **O**wnership **B**ased **R**esource **M**anagement is a built-in language feature.
Ownership rules are checked at compile time.
* Each value in Rust has a variable that is called its owner
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```
void memory_example() {
    let car = Box::new(Car{});
    let my_string = String::from("LGR");
    let car2 = car; //move is implicit
    function_that_can_throw();
    if(!should_continue()) return;
}

void memory_example() {
    let car = Rc::new(Car{}); // Reference counter implement.
    let car2 = car.clone();
    function_that_can_throw();
    if(!should_continue()) return;
}
```