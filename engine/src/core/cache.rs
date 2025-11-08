use lru::LruCache;
use parking_lot::Mutex;
use std::num::NonZeroUsize;

/// Thread-safe LRU cache for autocomplete results
pub struct Cache {
    inner: Mutex<LruCache<String, Vec<String>>>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(10000).unwrap());
        Self {
            inner: Mutex::new(LruCache::new(cap)),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<String>> {
        self.inner.lock().get(key).cloned()
    }

    pub fn put(&self, key: String, value: Vec<String>) {
        self.inner.lock().put(key, value);
    }

    pub fn clear(&self) {
        self.inner.lock().clear();
    }

    pub fn len(&self) -> usize {
        self.inner.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.lock().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache = Cache::new(2);

        cache.put("key1".to_string(), vec!["value1".to_string()]);
        assert_eq!(cache.get("key1"), Some(vec!["value1".to_string()]));

        cache.put("key2".to_string(), vec!["value2".to_string()]);
        assert_eq!(cache.len(), 2);

        // This should evict key1
        cache.put("key3".to_string(), vec!["value3".to_string()]);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("key1"), None);
    }
}
