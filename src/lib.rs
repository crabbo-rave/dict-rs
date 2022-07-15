use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct Dict<V> {
    table: Vec<Option<V>>,
}

impl<V> Dict<V> {
    pub fn new(n: usize) -> Self {
        Self {
            table: std::iter::repeat_with(|| None).take(n).collect(),
        }
    }

    pub fn set<K: Hash>(&mut self, key: K, value: V) {
        let hashed = self.hash_key(&key);
        match self.table.get(hashed) {
            Some(_) => self.table[hashed] = Some(value),
            None => self.table[hashed] = Some(value),
        }
    }

    pub fn get<K: Hash>(&self, key: &K) -> Option<&V> {
        match self.table.get(self.hash_key(key)) {
            Some(Some(value)) => Some(&value),
            Some(None) | None => None,
        }
    }

    fn hash_key<K: Hash>(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        (s.finish() % 1000).try_into().unwrap()
    }
}

impl<V: std::fmt::Display> Dict<V> {
    pub fn print<K: Hash>(&self, item: &K) {
        match self.get(item) {
            None => println!("key not found"),
            Some(value) => println!("{}", *value),
        }
    }
}
