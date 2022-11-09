use std::collections::VecDeque;
use tokio::time::Instant;

pub struct RateLimiter {
    track: VecDeque<Instant>,
    count: usize,
    duration: usize,
}

impl RateLimiter {
    pub fn new(count: usize, duration: usize) -> Self {
        Self {
            track: VecDeque::new(),
            count,
            duration
        }
    }

    pub async fn wait(&self) {

    }
}
