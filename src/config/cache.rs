// use cached::{Cached, SizedCache};
// use std::sync::Mutex;

// pub struct Cache {
//     storage: Mutex<SizedCache<String, bool>>,
// }

// impl Cache {
//     // Create a new Cache instance with a fixed size
//     pub fn new(size: usize) -> Self {
//         Cache {
//             storage: Mutex::new(SizedCache::with_size(size)),
//         }
//     }

//     // Check if a key exists in the cache
//     pub fn has(&self, key: &str) -> bool {
//         let mut cache = self.storage.lock().unwrap();
//         let result = cache.cache_get(key).is_some();
//         println!("HAS CCHE KEY: {}", result);
//         result
//     }

//     // Optionally, add a method to insert a value into the cache
//     pub fn insert(&self, key: &str, value: bool) {
//         let mut cache = self.storage.lock().unwrap();
//         cache.cache_set(key.to_string(), value);
//     }
// }
