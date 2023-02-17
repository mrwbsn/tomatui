use std::time::{Duration, Instant};

use crate::config;
use linya::Progress;

pub struct Pomo {
    pub config: config::Config,
    pub phase: Phase,
}

pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Pomo {
    pub fn new(config: config::Config) -> Self {
        Pomo {
            config,
            phase: Phase::Work,
        }
    }

    pub fn check(&mut self, work_cycle: u32) {
        match self.phase {
            Phase::Work => self.work(work_cycle),
            Phase::ShortBreak => self.short_break(work_cycle),
            Phase::LongBreak => self.long_break(),
        }
    }

    fn work(&mut self, mut work_cycle: u32) {
        self.run(self.config.work_duration);
        *self = if work_cycle < 3 {
            Pomo {
                config: self.config,
                phase: Phase::ShortBreak,
            }
        } else {
            Pomo {
                config: self.config,
                phase: Phase::LongBreak,
            }
        };

        work_cycle += 1;
        self.check(work_cycle)
    }

    fn short_break(&mut self, work_cycle: u32) {
        self.run(self.config.short_break_duration);
        *self = Pomo {
            config: self.config,
            phase: Phase::Work,
        };
        self.check(work_cycle)
    }

    fn long_break(&mut self) {
        self.run(self.config.long_break_duration);
        *self = Pomo {
            config: self.config,
            phase: Phase::Work,
        };
        // work_cycle reset to 0
        self.check(0)
    }

    fn run(&self, duration: u64) {
        let label: String = match self.phase {
            Phase::Work => "Work!".to_owned(),
            Phase::ShortBreak => "Short Break!".to_owned(),
            Phase::LongBreak => "Long break!".to_owned(),
        };

        let end_time = Instant::now() + Duration::from_secs(duration * 60);

        let mut progress = Progress::new();
        let bar = progress.bar(100, label);

        while Instant::now() < end_time {
            let remaining = end_time
                .checked_duration_since(Instant::now())
                .unwrap_or_else(|| Duration::from_secs(0));

            let percentage = 100.0
                - ((100.0 / Duration::from_secs(duration * 60).as_secs_f64())
                    * remaining.as_secs_f64());

            let percent_value = percentage.round() as usize;

            progress.set_and_draw(&bar, percent_value);

            std::thread::sleep(Duration::from_secs(10));
        }
    }
}
