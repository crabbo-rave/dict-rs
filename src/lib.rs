pub mod dict {
    pub struct Dict<K, V> {
        table: Vec<(K, V)>
    } 

    impl<K: std::cmp::PartialEq, V> Dict<K, V> {
        pub fn new() -> Self {
            Self { table: vec![] }
        }
        
        pub fn set(&mut self, key: K, value: V) {
            self.table.push((key, value));
        }

        pub fn get(&self, item: &K) -> Option<&V> {
            self.table
                .iter()
                .filter_map(|(key, value)| (*key == *item).then(|| value))
                .last()
        }
    }

    impl<K: std::cmp::PartialEq, V : std::fmt::Display> Dict<K, V> {
        pub fn print(&self, item: &K) {
            match self.get(item) {
                None => println!("key not found"),
                Some(value) => println!("{}", *value)
            }
        }
    }
}

