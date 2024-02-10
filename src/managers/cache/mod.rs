use std::collections::HashMap;
use crate::structs::timestamp::Timestamp;

trait CacheData: Clone {}

#[derive(Debug)]
/// A wrapper for cached data which contains information
pub struct Cache<CacheData> {
    last_updated: Timestamp,
    data: CacheData
}

impl<CacheData> Cache<CacheData> {
    pub fn new(d: CacheData) -> Self {
        Self {
            last_updated: Timestamp::now(),
            data: d
        }
    }

    pub fn update(&mut self, d: CacheData) {
        self.last_updated = Timestamp::now();
        self.data = d;
    }

    pub fn get(&self) -> &CacheData {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut CacheData {
        &mut self.data
    }
}

#[derive(Debug)]
pub struct CacheManager<CacheData> {
    cache: HashMap<String, Cache<CacheData>>,
}

impl<CacheData> CacheManager<CacheData> {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: CacheData) {
        self.cache.insert(key, Cache::new(value));
    }

    pub fn get(&self, key: &str) -> Option<&CacheData> {
        if self.has(key) {
            return Some(self.get(key)?)
        }

        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut CacheData> {
        if self.has(key) {
            let cache = self.cache.get_mut(key).unwrap();
            let data = cache.get_mut();
            return Some(data);
        }

        None
    }

    pub fn has(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Cache<CacheData>> {
        self.cache.remove(key)
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}