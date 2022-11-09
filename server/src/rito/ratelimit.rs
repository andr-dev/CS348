use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    track: Arc<Mutex<Instant>>,
    count: u64,
    duration: u64,
}

impl RateLimiter {
    pub fn new(count: u64, duration: u64) -> Self {
        Self {
            track: Arc::new(Mutex::new(Instant::now())),
            count,
            duration
        }
    }

    pub async fn wait(&self) {
        loop {
            let mut lock = self.track.lock().unwrap();

            let secs = lock.elapsed().as_secs();

            if secs > self.duration {
                *lock = Instant::now();
                break;
            } else {
                drop(lock);
                tokio::time::sleep(Duration::from_secs(self.duration - secs));
            }
        }
    }
}
