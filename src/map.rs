//! Empty maps. Dark, and alone.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::Empty;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A map that is guaranteed to be empty.
///
/// Naturally, the following operations are impossible with [`EMap`]:
///
/// - `entry`
/// - `insert`
/// - `remove`
///
/// And many others have been elided for being pointless.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(bound(
        serialize = "K: Eq + Hash + Clone + Serialize, V: Clone + Serialize ",
        deserialize = "K: Eq + Hash + Clone + Deserialize<'de>, V: Deserialize<'de>"
    )),
    serde(into = "HashMap<K, V>", try_from = "HashMap<K, V>")
)]
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EMap<K, V> {
    key: PhantomData<K>,
    val: PhantomData<V>,
}

impl<K, V> EMap<K, V> {
    /// No it doesn't.
    pub const fn contains_key<Q>(&self, _: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        false
    }

    /// All the keys of this empty map.
    pub fn keys(&self) -> Empty<&K> {
        Empty::new()
    }

    /// All the keys of this empty map.
    pub fn into_keys(self) -> Empty<K> {
        Empty::new()
    }

    /// All the values of this empty map.
    pub fn into_values(self) -> Empty<V> {
        Empty::new()
    }

    /// Yes.
    pub const fn is_empty(&self) -> bool {
        true
    }

    /// All key-value pairs of this empty map.
    pub fn iter(&self) -> Empty<(K, V)> {
        Empty::new()
    }

    /// It's 0.
    pub const fn len(&self) -> usize {
        0
    }

    /// Think of the possibilities!
    pub fn new() -> EMap<K, V> {
        EMap {
            key: PhantomData,
            val: PhantomData,
        }
    }

    /// All the values of this empty map.
    pub fn values(&self) -> Empty<&V> {
        Empty::new()
    }
}

impl<K, V> IntoIterator for EMap<K, V> {
    type Item = (K, V);

    type IntoIter = Empty<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        Empty::new()
    }
}

impl<K, V> From<EMap<K, V>> for HashMap<K, V> {
    fn from(_: EMap<K, V>) -> Self {
        HashMap::new()
    }
}

impl<K, V> TryFrom<HashMap<K, V>> for EMap<K, V> {
    type Error = &'static str;

    fn try_from(value: HashMap<K, V>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(EMap::new())
        } else {
            Err("Cannot convert a non-empty HashMap into an empty one")
        }
    }
}

impl<K, V> std::fmt::Debug for EMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: 2024-08-29 Critical inefficiency here.
        let m: HashMap<usize, usize> = HashMap::new();
        m.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let a: EMap<usize, usize> = EMap::new();
        let b = EMap::new();
        assert_eq!(a, b);
    }

    #[test]
    fn ord() {
        let a: EMap<usize, usize> = EMap::new();
        let b = EMap::new();
        assert!(!(a < b));
        assert!(!(a > b));
    }

    #[cfg(feature = "serde")]
    mod serialize {
        use crate::EMap;

        #[test]
        fn roundtrip() {
            let a: EMap<usize, usize> = EMap::new();
            let j = serde_json::to_string(&a).unwrap();
            let r = serde_json::from_str(&j).unwrap();
            assert_eq!(a, r);
        }
    }
}
