use std::time::{Duration, Instant};

/* Clamp framerate to a specific value.
 */
pub struct RateLimiter {
    // seconds per frame in ms
    available_time: Duration,
    previous: Instant,
}

impl RateLimiter {
    pub fn new(fps:u64) -> RateLimiter {
        RateLimiter {
            available_time: Duration::from_millis(1000 / fps),
            previous: Instant::now(),
        }
    }

    pub fn limit(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.previous);

        if self.available_time > duration {
            std::thread::sleep(self.available_time - duration);
        }

        self.previous = Instant::now();
    }
}
