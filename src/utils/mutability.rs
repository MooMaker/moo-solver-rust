use std::{
    collections::BTreeMap,
};

/// An utility function to get two mutable references to two different values in a BTreeMap.
/// Use with caution, as it will panic in case `a` and `b` pointing to the same value
pub fn get_mut_pair<'a, K, V>(map: &'a mut BTreeMap<K, V>, a: &K, b: &K) -> (&'a mut V, &'a mut V)
    where
        K: Eq + std::hash::Hash + std::cmp::Ord,
{
    unsafe {
        let a = map.get_mut(a).unwrap() as *mut _;
        let b = map.get_mut(b).unwrap() as *mut _;
        assert_ne!(a, b, "The two keys must not resolve to the same value");
        (&mut *a, &mut *b)
    }
}
