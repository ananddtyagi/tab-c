use std::collections::HashMap;

#[derive(Default)]
pub struct PrefixCache {
    capacity: usize,
    entries: HashMap<String, Vec<String>>,
}

impl PrefixCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, prefix: &str) -> Option<&Vec<String>> {
        self.entries.get(prefix)
    }

    pub fn insert(&mut self, prefix: String, candidates: Vec<String>) {
        if self.entries.len() >= self.capacity {
            if let Some(key) = self.entries.keys().next().cloned() {
                self.entries.remove(&key);
            }
        }
        self.entries.insert(prefix, candidates);
    }
}
