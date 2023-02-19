use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use nohash_hasher::BuildNoHashHasher;

pub(crate) fn hash<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::default();
    t.hash(&mut h);
    h.finish()
}

pub(crate) struct RefMap<K: Hash, V> {
    inner: HashMap<u64, V, BuildNoHashHasher<u64>>,
    _p: PhantomData<K>,
}

impl<K: Hash, V> RefMap<K, V> {
    pub(crate) fn new() -> RefMap<K, V> {
        RefMap {
            inner: HashMap::default(),
            _p: PhantomData,
        }
    }

    pub(crate) fn insert(&mut self, k: &K, v: V) -> Option<V> {
        self.inner.insert(hash(k), v)
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        self.inner.get(&hash(k))
    }
}
