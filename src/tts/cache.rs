/*!
 * TTS缓存实现
 */

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::tts::client::AudioData;

/// TTS缓存条目
#[derive(Debug, Clone)]
struct CacheEntry {
    data: AudioData,
    created_at: Instant,
    access_count: u32,
    last_accessed: Instant,
}

/// TTS缓存
#[derive(Debug)]
pub struct TTSCache {
    entries: HashMap<String, CacheEntry>,
    max_size: usize,
    max_age: Duration,
}

impl TTSCache {
    /// 创建新的缓存
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
            max_age: Duration::from_secs(3600), // 默认1小时过期
        }
    }

    /// 设置最大缓存时间
    pub fn with_max_age(mut self, max_age: Duration) -> Self {
        self.max_age = max_age;
        self
    }

    /// 获取缓存数据
    pub fn get(&mut self, key: &str) -> Option<AudioData> {
        // 清理过期条目
        self.cleanup_expired();

        if let Some(entry) = self.entries.get_mut(key) {
            entry.access_count += 1;
            entry.last_accessed = Instant::now();
            Some(entry.data.clone())
        } else {
            None
        }
    }

    /// 插入缓存数据
    pub fn insert(&mut self, key: String, data: AudioData) {
        let now = Instant::now();

        // 如果缓存已满，移除最少使用的条目
        if self.entries.len() >= self.max_size {
            self.evict_lru();
        }

        let entry = CacheEntry {
            data,
            created_at: now,
            access_count: 1,
            last_accessed: now,
        };

        self.entries.insert(key, entry);
    }

    /// 移除缓存条目
    pub fn remove(&mut self, key: &str) -> Option<AudioData> {
        self.entries.remove(key).map(|entry| entry.data)
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// 获取缓存大小
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 检查缓存是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 获取缓存容量
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// 设置缓存容量
    pub fn set_capacity(&mut self, capacity: usize) {
        self.max_size = capacity;
        
        // 如果当前大小超过新容量，移除多余条目
        while self.entries.len() > self.max_size {
            self.evict_lru();
        }
    }

    /// 获取缓存命中率
    pub fn hit_rate(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }

        let total_accesses: u32 = self.entries.values().map(|e| e.access_count).sum();
        let unique_entries = self.entries.len() as u32;
        
        if total_accesses == 0 {
            0.0
        } else {
            (total_accesses - unique_entries) as f64 / total_accesses as f64
        }
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> CacheStats {
        let now = Instant::now();
        let total_accesses: u32 = self.entries.values().map(|e| e.access_count).sum();
        let total_size: usize = self.entries.values()
            .map(|e| e.data.data.len())
            .sum();

        let avg_age = if self.entries.is_empty() {
            Duration::from_secs(0)
        } else {
            let total_age: Duration = self.entries.values()
                .map(|e| now.duration_since(e.created_at))
                .sum();
            total_age / self.entries.len() as u32
        };

        CacheStats {
            entries: self.entries.len(),
            capacity: self.max_size,
            total_accesses,
            hit_rate: self.hit_rate(),
            total_size_bytes: total_size,
            average_age: avg_age,
        }
    }

    /// 清理过期条目
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        let max_age = self.max_age;

        self.entries.retain(|_, entry| {
            now.duration_since(entry.created_at) < max_age
        });
    }

    /// 移除最少使用的条目（LRU）
    fn evict_lru(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        // 找到最少使用的条目
        let lru_key = self.entries
            .iter()
            .min_by(|(_, a), (_, b)| {
                // 首先按访问次数排序，然后按最后访问时间排序
                a.access_count.cmp(&b.access_count)
                    .then(a.last_accessed.cmp(&b.last_accessed))
            })
            .map(|(key, _)| key.clone());

        if let Some(key) = lru_key {
            self.entries.remove(&key);
        }
    }

    /// 预热缓存（预加载常用数据）
    pub fn warmup(&mut self, common_texts: Vec<(String, AudioData)>) {
        for (key, data) in common_texts {
            if self.entries.len() < self.max_size {
                self.insert(key, data);
            } else {
                break;
            }
        }
    }

    /// 获取最热门的缓存条目
    pub fn get_hot_entries(&self, limit: usize) -> Vec<(String, u32)> {
        let mut entries: Vec<_> = self.entries
            .iter()
            .map(|(key, entry)| (key.clone(), entry.access_count))
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(limit);
        entries
    }

    /// 获取最近访问的缓存条目
    pub fn get_recent_entries(&self, limit: usize) -> Vec<(String, Instant)> {
        let mut entries: Vec<_> = self.entries
            .iter()
            .map(|(key, entry)| (key.clone(), entry.last_accessed))
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(limit);
        entries
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub capacity: usize,
    pub total_accesses: u32,
    pub hit_rate: f64,
    pub total_size_bytes: usize,
    pub average_age: Duration,
}

impl CacheStats {
    /// 格式化为可读字符串
    pub fn format(&self) -> String {
        format!(
            "缓存统计: {}/{} 条目, {:.1}% 命中率, {} 次访问, {:.1} KB, 平均年龄 {:.1}s",
            self.entries,
            self.capacity,
            self.hit_rate * 100.0,
            self.total_accesses,
            self.total_size_bytes as f64 / 1024.0,
            self.average_age.as_secs_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tts::client::AudioFormat;

    fn create_test_audio_data() -> AudioData {
        AudioData {
            data: vec![1, 2, 3, 4, 5],
            format: AudioFormat::Wav,
            duration: Duration::from_secs(1),
            sample_rate: 44100,
        }
    }

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = TTSCache::new(2);
        let audio_data = create_test_audio_data();

        // 测试插入和获取
        cache.insert("key1".to_string(), audio_data.clone());
        assert_eq!(cache.len(), 1);

        let retrieved = cache.get("key1");
        assert!(retrieved.is_some());

        // 测试不存在的键
        let not_found = cache.get("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_cache_lru_eviction() {
        let mut cache = TTSCache::new(2);
        let audio_data = create_test_audio_data();

        // 填满缓存
        cache.insert("key1".to_string(), audio_data.clone());
        cache.insert("key2".to_string(), audio_data.clone());
        assert_eq!(cache.len(), 2);

        // 访问key1使其成为最近使用的
        cache.get("key1");

        // 插入新条目应该移除key2（最少使用的）
        cache.insert("key3".to_string(), audio_data.clone());
        assert_eq!(cache.len(), 2);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!(cache.get("key3").is_some());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = TTSCache::new(10);
        let audio_data = create_test_audio_data();

        cache.insert("key1".to_string(), audio_data.clone());
        cache.insert("key2".to_string(), audio_data.clone());
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = TTSCache::new(10);
        let audio_data = create_test_audio_data();

        cache.insert("key1".to_string(), audio_data.clone());
        cache.get("key1"); // 第二次访问

        let stats = cache.stats();
        assert_eq!(stats.entries, 1);
        assert_eq!(stats.capacity, 10);
        assert_eq!(stats.total_accesses, 2);
        assert!(stats.hit_rate > 0.0);
    }
}
