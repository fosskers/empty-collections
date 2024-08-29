//! Empty sets. Sets of empty sets. Sets of sets of empty sets.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::Empty;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::marker::PhantomData;

/// A set that is guaranteed to be empty.
///
/// Naturally, the following operations are impossible with [`ESet`]:
///
/// - `insert`
/// - `remove`
///
/// And many others have been elided for being pointless.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(bound(
        serialize = "T: Eq + Hash + Clone + Serialize",
        deserialize = "T: Eq + Hash + Deserialize<'de>"
    )),
    serde(into = "HashSet<T>", try_from = "HashSet<T>")
)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ESet<T> {
    val: PhantomData<T>,
}

impl<T> ESet<T> {
    /// No it doesn't.
    pub const fn contains<Q>(&self, _: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        false
    }

    /// What would a set type be without set operations?
    pub fn difference(&self, _: &ESet<T>) -> Empty<&T> {
        Empty::new()
    }

    /// Elements only present in both sets. Wouldn't it be funny if something
    /// popped out of this?
    pub fn intersection(&self, _: &ESet<T>) -> Empty<&T> {
        Empty::new()
    }

    /// You never want the book to end, but sometimes it has to.
    pub fn iter(&self) -> Empty<&T> {
        Empty::new()
    }

    /// Yes.
    pub const fn is_empty(&self) -> bool {
        true
    }

    /// It's 0.
    pub const fn len(&self) -> usize {
        0
    }

    /// Think of the possibilities!
    pub fn new() -> ESet<T> {
        ESet { val: PhantomData }
    }

    /// All elements in both sets. _All_ of them.
    pub fn union(&self, _: &ESet<T>) -> Empty<&T> {
        Empty::new()
    }
}

impl<T> IntoIterator for ESet<T> {
    type Item = T;

    type IntoIter = Empty<T>;

    fn into_iter(self) -> Self::IntoIter {
        Empty::new()
    }
}

impl<T> From<ESet<T>> for HashSet<T> {
    fn from(_: ESet<T>) -> Self {
        HashSet::new()
    }
}

impl<T> TryFrom<HashSet<T>> for ESet<T> {
    type Error = &'static str;

    fn try_from(value: HashSet<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(ESet::new())
        } else {
            Err("Cannot convert a non-empty HashSet into an empty one")
        }
    }
}

impl<T> std::fmt::Debug for ESet<T> {
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
        let a: ESet<usize> = ESet::new();
        let b = ESet::new();
        assert_eq!(a, b);
    }

    #[test]
    fn ord() {
        let a: ESet<usize> = ESet::new();
        let b = ESet::new();
        assert!(!(a < b));
        assert!(!(a > b));
    }

    #[cfg(feature = "serde")]
    mod serialize {
        use crate::ESet;

        #[test]
        fn roundtrip() {
            let a: ESet<usize> = ESet::new();
            let j = serde_json::to_string(&a).unwrap();
            let r = serde_json::from_str(&j).unwrap();
            assert_eq!(a, r);
        }
    }
}
