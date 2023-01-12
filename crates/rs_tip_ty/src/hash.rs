use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub(crate) fn hash<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::default();
    t.hash(&mut h);
    h.finish()
}
