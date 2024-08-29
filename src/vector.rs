//! Empty vectors. An icy wind blows across the wintering fields of Manitoba.
//!
//! Naturally, the following operations are impossible with [`EVec`]:
//!
//! - `append`
//! - `insert`
//! - `push`
//! - `remove`
//!
//! And many others have been elided for being pointless (like `drain` and `sort`).

use std::marker::PhantomData;

/// A vector that is guaranteed to be empty.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EVec<T> {
    // Spooky. Don't look behind you.
    vector: PhantomData<T>,
}

impl<T> EVec<T> {
    /// Go ahead, see if it's in there.
    pub const fn contains(&self, _: &T) -> bool {
        false
    }

    /// Attempt to convert a possibly non-empty `Vec` into one that is
    /// guaranteed to be empty.
    pub fn from_vec(vec: Vec<T>) -> Option<EVec<T>> {
        vec.is_empty().then(|| EVec::new())
    }

    /// This may be the best function I've ever written. Go on, guess the return
    /// value.
    pub const fn is_empty(&self) -> bool {
        true
    }

    /// No, sorry, _this_ is the best function I've ever written.
    pub const fn len(&self) -> usize {
        0
    }

    /// Think of the possibilities!
    pub fn new() -> EVec<T> {
        EVec {
            vector: PhantomData,
        }
    }
}

impl<T> From<EVec<T>> for Vec<T> {
    fn from(_: EVec<T>) -> Self {
        Vec::new()
    }
}

impl<T> std::fmt::Debug for EVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: [usize; 0] = [];
        v.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let a: EVec<usize> = EVec::new();
        let b = EVec::new();
        assert_eq!(a, b);
    }

    #[test]
    fn ord() {
        let a: EVec<usize> = EVec::new();
        let b = EVec::new();
        assert!(!(a < b));
        assert!(!(a > b));
    }
}
