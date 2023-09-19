# String Types

Character Encoding and length

ASCII - 1 byte and can support english, symbols and control characters
UTF-8 - 1 to 4 bytes and can support over a million character and is backward compatible with ASCII.

String is a sequence of bytes that sits within a larger block of memory. When we create a string, it refers to the address of the first character of the string. The length of string can be determined either by a termination character to mark the end of the string, or by storing the length of the string along with the address of the first character of the string in a higher level data structure.

While C uses termination character to mark the end of the string ("\0"), Rust re-imagines string for safety, efficiency and flexibility. It does so by -

* Rust saves the length of string in a metadata and does not use null character.
* Rust guarantees that all strings are valid UTF-8.
* String are immutable by default.

There are many ways to represent strings in Rust - `Cow<'a, str>`, `OsString`, `Path`, `String`, `&str`, `OsStr`, `CStr`,`Vec<u8>`, `&[u8; N]`, etc. There are two primary String types in Rust - `String` and `&str`

String in Rust is a heap allocated, growable, UTF-8 encoded string. It is owned type, since it owns the underlying data, and is responsible to cleaning it up. This type has **pointer to the string**, **length** and **its capacity**.

&str, also called string slice is a view to a string. This type has **pointer to the string**, and **length**. It is called a burrowed type, because it does not own the underlying data, and simply has access to it. String slice can access the access on the heap, or the data section in the binary or the stack itself.

String is useful when you want to create / modify string data dynamically at runtime. String slice is useful when you want to read or analyze pre-existing string data.

Other string types provide the efficiency and flexibility that we occasionally need.

* `& str`, a syntactic sugarcoat for `&'static str` is used for literal. `let hello: &str = "Hello, world!`.
* `Box<str>` represents an owned, non-growable, heap-allocated string slice. It is useful if you want to restrict further modification of string or save memory.

```rust
let my_string: String = String::from("This is a long string that I don't intend to modify further");
let my_boxed_str: Box<str> = my_string.into_boxed_str();
println!("My boxed str: {}", my_boxed_str)
```

* `Rc<str>` is a shared, immutable string slice.

## Source

* [All Rust string types explained](https://www.youtube.com/watch?v=CpvzeyzgQdw)
