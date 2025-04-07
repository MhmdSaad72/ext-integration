use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct Cache {
    pub store: Arc<Mutex<HashMap<String, CacheEntry>>>,
}
#[derive(Debug)]
pub struct CacheEntry {
    pub _value: bool,
    pub expires_at: Instant,
}

impl Cache {
    pub fn new() -> Self {
        let service = Cache {
            store: Arc::new(Mutex::new(HashMap::new())),
        };
        service.cleanup();
        service
    }

    pub fn insert(&self, key: &str, ttl_seconds: u64) {
        let mut store = self.store.lock().unwrap();
        let expires_at = Instant::now() + Duration::from_secs(ttl_seconds);
        let entry = CacheEntry {
            _value: true,
            expires_at,
        };
        store.insert(key.to_string(), entry);
    }

    pub fn has(&self, key: &str) -> bool {
        let store = self.store.lock().unwrap();
        if let Some(entry) = store.get(key) {
            println!("Cache entry found:");
            if entry.expires_at > Instant::now() {
                return true;
            }
        }
        false
    }

    fn cleanup(&self) {
        let store = self.store.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut store = store.lock().unwrap();
                let now = Instant::now();
                store.retain(|_, entry| entry.expires_at > now);
            }
        });
    }
}
