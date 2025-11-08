use parking_lot::RwLock;
use std::collections::VecDeque;

use crate::ipc::protocol::MetricsData;

const MAX_LATENCY_SAMPLES: usize = 1000;

pub struct Metrics {
    inner: RwLock<MetricsInner>,
}

struct MetricsInner {
    total_requests: u64,
    instant_latencies: VecDeque<u64>,
    refined_latencies: VecDeque<u64>,
    cache_hits: u64,
    cache_misses: u64,
    accepts: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(MetricsInner {
                total_requests: 0,
                instant_latencies: VecDeque::new(),
                refined_latencies: VecDeque::new(),
                cache_hits: 0,
                cache_misses: 0,
                accepts: 0,
            }),
        }
    }

    pub fn record_request(&self) {
        self.inner.write().total_requests += 1;
    }

    pub fn record_instant_latency(&self, latency_ms: u64) {
        let mut inner = self.inner.write();
        inner.instant_latencies.push_back(latency_ms);
        if inner.instant_latencies.len() > MAX_LATENCY_SAMPLES {
            inner.instant_latencies.pop_front();
        }
    }

    pub fn record_refined_latency(&self, latency_ms: u64) {
        let mut inner = self.inner.write();
        inner.refined_latencies.push_back(latency_ms);
        if inner.refined_latencies.len() > MAX_LATENCY_SAMPLES {
            inner.refined_latencies.pop_front();
        }
    }

    pub fn record_cache_hit(&self) {
        self.inner.write().cache_hits += 1;
    }

    pub fn record_cache_miss(&self) {
        self.inner.write().cache_misses += 1;
    }

    pub fn record_accept(&self) {
        self.inner.write().accepts += 1;
    }

    pub fn snapshot(&self) -> MetricsData {
        let inner = self.inner.read();

        let instant_p50 = Self::percentile(&inner.instant_latencies, 50);
        let instant_p95 = Self::percentile(&inner.instant_latencies, 95);
        let refined_p50 = Self::percentile(&inner.refined_latencies, 50);
        let refined_p95 = Self::percentile(&inner.refined_latencies, 95);

        let total_cache_ops = inner.cache_hits + inner.cache_misses;
        let cache_hit_rate = if total_cache_ops > 0 {
            inner.cache_hits as f32 / total_cache_ops as f32
        } else {
            0.0
        };

        let accept_rate = if inner.total_requests > 0 {
            inner.accepts as f32 / inner.total_requests as f32
        } else {
            0.0
        };

        MetricsData {
            total_requests: inner.total_requests,
            instant_latency_p50_ms: instant_p50,
            instant_latency_p95_ms: instant_p95,
            refined_latency_p50_ms: refined_p50,
            refined_latency_p95_ms: refined_p95,
            accept_rate,
            cache_hit_rate,
        }
    }

    fn percentile(values: &VecDeque<u64>, percentile: usize) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<u64> = values.iter().copied().collect();
        sorted.sort_unstable();

        let index = (sorted.len() * percentile / 100).min(sorted.len() - 1);
        sorted[index] as f64
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let metrics = Metrics::new();

        metrics.record_request();
        metrics.record_instant_latency(10);
        metrics.record_refined_latency(200);
        metrics.record_cache_hit();
        metrics.record_accept();

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.instant_latency_p50_ms, 10.0);
        assert_eq!(snapshot.cache_hit_rate, 1.0);
    }
}
