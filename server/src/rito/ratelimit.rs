use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    track: Arc<Mutex<Instant>>,
    count: u128,
    duration: u128,
}

impl RateLimiter {
    pub fn new(count: u128, duration: u128) -> Self {
        Self {
            track: Arc::new(Mutex::new(Instant::now())),
            count,
            duration
        }
    }

    pub async fn wait(&self) {
        loop {
            let mut lock = self.track.lock().unwrap();

            let secs = lock.elapsed().as_millis();

            if secs > self.duration * 1000 / self.count {
                *lock = Instant::now();
                break;
            } else {
                drop(lock);
                tokio::time::sleep(Duration::from_millis((self.duration * 1000 - secs) as u64));
            }
        }
    }
}
