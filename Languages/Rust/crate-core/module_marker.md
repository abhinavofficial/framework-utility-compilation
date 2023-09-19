# Marker

## Traits

### Send

Types that can be transferred across thread boundaries.

This trait is automatically implemented when the compiler determines it’s appropriate.

An example of a non-Send type is the reference-counting pointer [rc::Rc](https://doc.rust-lang.org/src/alloc/rc.rs.html). If two threads attempt to clone [Rcs](https://doc.rust-lang.org/std/rc/struct.Rc.html) that point to the same reference-counted value, they might try to update the reference count at the same time, which is [undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) because Rc doesn’t use atomic operations. Its cousin [sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) does use atomic operations (incurring some overhead) and thus is Send.
