use std::collections::hash_map::HashMap;

pub struct KvStore {
    kv_pairs: HashMap<String, String>
}

impl KvStore {
    pub fn len(&self) -> usize {
        self.kv_pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.kv_pairs.len() == 0
    }

    pub fn new() -> Self {
        KvStore {
            kv_pairs: HashMap::new()
        }
    }

    pub fn set(&mut self, _key: String, _value: String) {
        panic!()
    }

    pub fn get(&self, _key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&self, _key: String) {
        panic!()
    }
}

