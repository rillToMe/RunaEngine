use std::time::{Duration, Instant};

pub struct Time {
    start_time: Instant,
    last_frame: Instant,

    delta: Duration,
    accumulator: Duration,

    frame_count: u64,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            start_time: now,
            last_frame: now,

            delta: Duration::ZERO,
            accumulator: Duration::ZERO,

            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta = now.duration_since(self.last_frame);
        self.last_frame = now;

        self.accumulator += self.delta;

        self.frame_count += 1;
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn fixed_delta_seconds() -> f32 {
        1.0 / 60.0
    }

    pub fn consume_fixed_step(&mut self) -> bool {
        let fixed_delta = Duration::from_secs_f32(Self::fixed_delta_seconds());

        if self.accumulator >= fixed_delta {
            self.accumulator -= fixed_delta;
            true
        } else {
            false
        }
    }   

    pub fn reset_accumulator(&mut self) {
        self.accumulator = Duration::ZERO;
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn fps(&self) -> f32 {
        let delta = self.delta_seconds();

        if delta > 0.0 {
            1.0 / delta
        } else {
            0.0
        }
    }
}