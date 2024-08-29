# empty-collections

<!-- cargo-rdme start -->

Empty collections - guaranteed to have nothing!

You may be familiar with the concept of [nonempty-collections][ne]. The
`empty-collections` crate provides the sister-concept; collections which
contain nothing, and never will.

Why, you ask? That is a good question.

## Examples

```rust
use empty_collections::*;

let v: EVec<usize> = EVec::new();
assert!(v.is_empty());
```

See the documentation for `EVec`, `EMap`, and `ESet` for more examples of their
extensive APIs.

## Iteration

The iterators in this crate are the fastest in the entire Rust ecosystem,
able to traverse their entire stream in constant time. Simply amazing.

```rust
use empty_collections::*;

let v: EVec<i32> = EVec::new();
assert_eq!(0, v.into_iter().sum());
```

## Features

- `serde`: Guarantee that collections you send/receive over the wire are empty.

[ne]: https://lib.rs/crates/nonempty-collections

<!-- cargo-rdme end -->
