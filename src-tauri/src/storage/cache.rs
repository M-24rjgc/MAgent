use std::collections::HashMap;
use dashmap::DashMap;
use anyhow::Result;

/// 内存缓存
pub struct MemoryCache {
    data: DashMap<String, CachedValue>,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct CachedValue {
    pub data: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
            max_size: 1000, // 默认最大1000个条目
        }
    }

    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            data: DashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(mut entry) = self.data.get_mut(key) {
            entry.access_count += 1;
            Some(entry.data.clone())
        } else {
            None
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<()> {
        let cached_value = CachedValue {
            data: value,
            created_at: chrono::Utc::now(),
            access_count: 0,
        };

        // Check if cache is full
        if self.data.len() >= self.max_size {
            self.evict_lru();
        }

        self.data.insert(key, cached_value);
        Ok(())
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        self.data.remove(key).map(|(_, v)| v.data)
    }

    pub fn clear(&self) {
        self.data.clear();
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    fn evict_lru(&self) {
        // Simple LRU eviction: remove the oldest item with lowest access count
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = chrono::Utc::now();
        let mut lowest_access = u64::MAX;

        for entry in self.data.iter() {
            let value = entry.value();
            if value.created_at < oldest_time || 
               (value.created_at == oldest_time && value.access_count < lowest_access) {
                oldest_time = value.created_at;
                lowest_access = value.access_count;
                oldest_key = Some(entry.key().clone());
            }
        }

        if let Some(key) = oldest_key {
            self.data.remove(&key);
            tracing::debug!("Evicted cache entry: {}", key);
        }
    }

    pub fn stats(&self) -> CacheStats {
        let mut total_access = 0;
        let mut oldest = chrono::Utc::now();
        let mut newest = chrono::DateTime::<chrono::Utc>::MIN_UTC;

        for entry in self.data.iter() {
            let value = entry.value();
            total_access += value.access_count;
            if value.created_at < oldest {
                oldest = value.created_at;
            }
            if value.created_at > newest {
                newest = value.created_at;
            }
        }

        CacheStats {
            size: self.data.len(),
            max_size: self.max_size,
            total_access_count: total_access,
            oldest_entry: oldest,
            newest_entry: newest,
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub total_access_count: u64,
    pub oldest_entry: chrono::DateTime<chrono::Utc>,
    pub newest_entry: chrono::DateTime<chrono::Utc>,
}
