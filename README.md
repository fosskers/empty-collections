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

// Unfortunately we have to specify a type.
let v: EVec<usize> = EVec::new();
assert!(v.is_empty());
```

See the documentation for `EVec` for more examples of what it (can't) do.

## Features

- `serde`: Guarantee that collections you send/receive over the wire are empty.

[ne]: https://lib.rs/crates/nonempty-collections

<!-- cargo-rdme end -->
