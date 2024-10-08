# Sync Module

## Arc

```rust
pub struct Arc<T> where
T: ?Sized,  { /* fields omitted */ }
```

A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’. The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking ```clone``` on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped.

Since Shared references in Rust disallow mutation by default, you cannot generally obtain a mutable reference to something inside an Arc. If you need to mutate through an Arc, use ```Mutex```, ```RwLock```, or one of the Atomic types.

### Thread Safety

Unlike ```Rc<T>```, ```Arc<T>``` uses atomic operations for its reference counting. This means that it is thread-safe. If you are not sharing reference-counted allocations between threads, consider using `Rc<T>` for lower overhead. `Rc<T>` is a safe default, because the compiler will catch any attempt to send an `Rc<T>` between threads. However, a library might choose `Arc<T>` in order to give library consumers more flexibility.

`Arc<T>` will implement ```Send``` and ```Sync``` as long as the T implements them. Why can’t you put a non-thread-safe type T in an `Arc<T>` to make it thread-safe? This may be a bit counter-intuitive at first: after all, isn’t the point of `Arc<T>` thread safety? The key is this: `Arc<T>` makes it thread safe to have multiple ownership of the same data, but it doesn’t add thread safety to its data. Consider `Arc<RefCell<T>>`. `RefCell<T>` isn’t Sync, and if `Arc<T>` was always Send, `Arc<RefCell<T>>` would be as well. But then we’d have a problem: `RefCell<T>` is not thread safe; it keeps track of the borrowing count using non-atomic operations. In the end, this means that **you may need to pair `Arc<T>` with some sort of `std::sync` type, usually `Mutex<T>`**.

### Breaking cycles with Weak

The ```downgrade``` method can be used to create a non-owning ```Weak``` pointer. A Weak pointer can be upgraded to an Arc, but this will return None if the value stored in the allocation has already been dropped. In other words, Weak pointers do not keep the value inside the allocation alive; however, they do keep the allocation (the backing store for the value) alive.

A cycle between Arc pointers will never be de-allocated. For this reason, Weak is used to break cycles. For example, a tree could have strong Arc pointers from parent nodes to children, and Weak pointers from children back to their parents.

### Cloning references

Creating a new reference from an existing reference-counted pointer is done using the Clone trait implemented for `Arc<T>` and `Weak<T>`.

```rust
use std::sync::Arc;
let foo = Arc::new(vec![1.0, 2.0, 3.0]);
// The two syntaxes below are equivalent.
let a = foo.clone();
let b = Arc::clone(&foo);
// a, b, and foo are all Arcs that point to the same memory location
```

### Deref behavior

`Arc<T>` automatically dereferences to T (via the ```Deref``` trait), so you can call T’s methods on a value of type `Arc<T>`. To avoid name clashes with T’s methods, the methods of `Arc<T>` itself are associated functions, called using ```fully qualified syntax```:

```rust
use std::sync::Arc;

let my_arc = Arc::new(());
let my_weak = Arc::downgrade(&my_arc);
```

`Arc<T>’s` implementations of traits like Clone may also be called using fully qualified syntax. Some people prefer to use fully qualified syntax, while others prefer using method-call syntax.

```rust
use std::sync::Arc;

let arc = Arc::new(());
// Method-call syntax
let arc2 = arc.clone();
// Fully qualified syntax
let arc3 = Arc::clone(&arc);
```

`Weak<T>` does not auto-dereference to T, because the inner value may have already been dropped.
