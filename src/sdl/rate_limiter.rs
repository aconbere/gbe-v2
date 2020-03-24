use sdl2;

/* Clamp framerate to a specific value.
 */
pub struct RateLimiter {
    last_ticks: u32,
    fps: u32,
}

pub fn new(fps: u32) -> RateLimiter {
    RateLimiter {
        fps: fps,
        last_ticks: 0,
    }
}

impl RateLimiter {
    pub fn limit(&mut self, timer: &mut sdl2::TimerSubsystem) {
        let ticks = timer.ticks();
        let adjusted_ticks = ticks - self.last_ticks;
        if adjusted_ticks < 1000 / self.fps {
            timer.delay((1000 / self.fps) - adjusted_ticks);
        }
        self.last_ticks = ticks;
    }
}
