/// #![deny(missing_docs)]
use std::collections::hash_map::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    kv_pairs: HashMap<String, String>,
}

impl KvStore {
    /// return store's len
    pub fn len(&self) -> usize {
        self.kv_pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.kv_pairs.is_empty()
    }

    pub fn new() -> Self {
        KvStore {
            kv_pairs: HashMap::new(),
        }
    }

    pub fn set(&mut self, _key: String, _value: String) {
        self.kv_pairs.insert(_key, _value);
    }

    pub fn get(&self, _key: String) -> Option<String> {
        self.kv_pairs.get(&_key).cloned()
    }

    pub fn remove(&mut self, _key: String) {
        self.kv_pairs.remove(&_key);
    }
}
