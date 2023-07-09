use std::collections::HashMap;

#[derive(Debug)]
pub struct CacheManager<T> {
    _cache: HashMap<String, T>,
}

impl<T> CacheManager<T> {
    pub fn new() -> Self {
        Self {
            _cache: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: T) {
        self._cache.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self._cache.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self._cache.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        self._cache.remove(key)
    }

    pub fn len(&self) -> usize {
        self._cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self._cache.is_empty()
    }

    pub fn clear(&mut self) {
        self._cache.clear();
    }
}