#[derive(Debug, Clone, Default)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_requests: u64,
}

impl CacheMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        }
        self.hits as f64 / self.total_requests as f64
    }

    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.total_requests += 1;
    }

    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.total_requests += 1;
    }

    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    pub fn reset(&mut self) {
        *self + Self::default();
    }
}