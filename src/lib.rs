//! Empty collections - guaranteed to have nothing!
//!
//! You may be familiar with the concept of [nonempty-collections][ne]. The
//! `empty-collections` crate provides the sister-concept; collections which
//! contain nothing, and never will.
//!
//! Why, you ask? That is a good question.
//!
//! # Examples
//!
//! ```
//! use empty_collections::*;
//!
//! let v: EVec<usize> = EVec::new();
//! assert!(v.is_empty());
//! ```
//!
//! See the documentation for [`EVec`], [`EMap`], and [`ESet`] for more examples
//! of their extensive APIs.
//!
//! # Iteration
//!
//! The iterators in this crate are the fastest in the entire Rust ecosystem,
//! able to traverse their entire stream in constant time. Simply amazing.
//!
//! ```
//! use empty_collections::*;
//!
//! let v: EVec<i32> = EVec::new();
//! assert_eq!(0, v.into_iter().sum());
//! ```
//!
//! # Features
//!
//! - `serde`: Guarantee that collections you send/receive over the wire are empty.
//!
//! [ne]: https://lib.rs/crates/nonempty-collections

#![warn(missing_docs)]

mod map;
mod set;
mod vector;

use std::marker::PhantomData;

pub use map::EMap;
pub use set::ESet;
pub use vector::EVec;

/// An empty [`Iterator`].
pub struct Empty<T> {
    phantom: PhantomData<T>,
}

impl<T> Empty<T> {
    pub(crate) fn new() -> Empty<T> {
        Empty {
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for Empty<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
