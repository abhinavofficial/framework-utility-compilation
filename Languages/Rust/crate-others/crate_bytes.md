# Crate Byte

Source: [Bytes - Version 1.1.0](https://docs.rs/bytes/1.1.0/bytes/)

Provides abstractions for working with bytes.
The bytes crate provides an efficient byte buffer structure (Bytes) and traits for working with buffer implementations (Buf, BufMut).

Bytes is an efficient container for storing and operating on contiguous slices of memory. It is intended for use primarily in networking code, but could have applications elsewhere as well. Its values facilitate zero-copy network programming by allowing multiple Bytes objects to point to the same underlying memory. This is managed by using a reference count to track when the memory is no longer needed and can be freed.
A Bytes handle can be created directly from an existing byte store (such as `&[u8]` or `Vec<u8>`), but usually a BytesMut is used first and written to.
