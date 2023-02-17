use crate::config;
use linya::Progress;

pub struct Tomato {
    pub config: config::Config,
    pub phase: Phase,
}

pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Tomato {
    pub fn new(config: config::Config) -> Self {
        Tomato {
            config,
            phase: Phase::Work,
        }
    }

    pub fn check(&mut self, work_cycle: i32) {
        match self.phase {
            Phase::Work => self.work(work_cycle),
            Phase::ShortBreak => self.short_break(work_cycle),
            Phase::LongBreak => self.long_break(),
        }
    }

    fn work(&mut self, mut work_cycle: i32) {
        self.run(self.config.work_duration);
        *self = if work_cycle < 5 {
            Tomato {
                config: self.config,
                phase: Phase::ShortBreak,
            }
        } else {
            Tomato {
                config: self.config,
                phase: Phase::LongBreak,
            }
        };

        work_cycle += 1;
        self.check(work_cycle)
    }

    fn short_break(&mut self, work_cycle: i32) {
        self.run(self.config.short_break_duration);
        *self = Tomato {
            config: self.config,
            phase: Phase::Work,
        };
        self.check(work_cycle)
    }

    fn long_break(&mut self) {
        self.run(self.config.long_break_duration);
        *self = Tomato {
            config: self.config,
            phase: Phase::Work,
        };
        // work_cycle reset to 0
        self.check(0)
    }

    fn run(&self, _duration: i32) {
        let mut progress = Progress::new();
        let bar = progress.bar(50, "Work!");

        progress.inc_and_draw(&bar, 50);
    }
}
