use clap::Parser;
use linya::Progress;
use std::io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of task
    task: Option<String>,
}

#[derive(Debug)]
enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Phase {
    fn new() -> Self {
        Phase::Work
    }

    fn check(&mut self, work_cycle: i32) {
        match self {
            Phase::Work => self.work(work_cycle),
            Phase::ShortBreak => self.short_break(work_cycle),
            Phase::LongBreak => self.long_break(),
        }
    }

    fn work(&mut self, mut work_cycle: i32) {
        let duration = 15;
        self.run(duration);
        if work_cycle < 5 {
            *self = Phase::ShortBreak;
        } else {
            *self = Phase::LongBreak;
        }
        work_cycle += 1;
        self.check(work_cycle)
    }

    fn short_break(&mut self, work_cycle: i32) {
        let duration = 5;
        self.run(duration);
        *self = Phase::Work;
        self.check(work_cycle)
    }

    fn long_break(&mut self) {
        let duration = 30;
        self.run(duration);
        *self = Phase::Work;
        // work_cycle reset to 0
        self.check(0)
    }

    fn run(&self, duration: i32) {
        println!("{duration}");
        let mut progress = Progress::new();
        let bar = progress.bar(50, "Work!");

        progress.inc_and_draw(&bar, 50);
    }
}

fn main() {
    let args = cli_args();
    cli_confirmation(args);

    let mut tomatui = Phase::new();
    tomatui.check(0);
}

fn cli_args() -> String {
    let cli = Cli::parse();
    let t;
    if let Some(task) = cli.task.as_deref() {
        t = task
    } else {
        t = "task"
    }

    t.to_owned()
}

fn cli_confirmation(args: String) {
    let mut ks = String::new();
    println!("Do you want to start [{args}] with Tomatui? [Y/n]");

    while ks != "Y" && ks != "y" {
        ks.clear();
        io::stdin().read_line(&mut ks).unwrap();
        ks = ks.trim().to_owned();

        if ks.is_empty() {
            break;
        }
    }
}
