pub mod dict {
    pub struct Dict<K: Eq, V> {
        table: Vec<(K, V)>
    } 

    impl<K: Eq, V> Dict<K, V> {
        pub fn new() -> Self {
            Self { table: vec![] }
        }
        
        pub fn set(&mut self, key: K, value: V) {
            match self.find_key(&key) {
                Some(idx) => self.table[idx] = (key, value),
                None => self.table.push((key, value))
            }
        }

        pub fn get(&self, item: &K) -> Option<&V> {
            match self.find_key(item) {
                Some(idx) => Some(&self.table[idx].1),
                None => None
            }
        }

        fn find_key(&self, item: &K) -> Option<usize> {
            self.table
                .iter()
                .position(|(key, _)| *key == *item)
        }
    }

    impl<K: Eq, V : std::fmt::Display> Dict<K, V> {
        pub fn print(&self, item: &K) {
            match self.get(item) {
                None => println!("key not found"),
                Some(value) => println!("{}", *value)
            }
        }
    }
}

