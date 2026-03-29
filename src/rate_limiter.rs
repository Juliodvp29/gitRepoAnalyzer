use dashmap::DashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: DashMap<String, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: DashMap::new(),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self, ip: &str) -> bool {
        let now = Instant::now();
        let mut entry = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);

        entry.retain(|t| now.duration_since(*t) < self.window);

        if entry.len() >= self.max_requests {
            return false;
        }

        entry.push(now);
        true
    }

    pub fn remaining(&self, ip: &str) -> usize {
        let now = Instant::now();
        let entry = self.requests.get(ip);
        match entry {
            None => self.max_requests,
            Some(times) => {
                let active = times.iter()
                    .filter(|t| now.duration_since(**t) < self.window)
                    .count();
                self.max_requests.saturating_sub(active)
            }
        }
    }
}