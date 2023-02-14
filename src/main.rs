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
    Work(i32),
    ShortBreak,
    LongBreak,
}

impl Phase {
    fn new() -> Self {
        Phase::Work(0)
    }

    fn check(&mut self) {
        match self {
            Phase::Work(..) => self.work(),
            Phase::ShortBreak => self.short_break(),
            Phase::LongBreak => self.long_break(),
        }
    }

    fn work(&mut self) {
        let duration = 15;
        self.run(duration);
        *self = Phase::ShortBreak;
        self.check()
    }

    fn short_break(&mut self) {
        let mut cycle = 0;
        if let Phase::Work(n) = self {
            cycle = *n + 1
        }
        println!("{cycle}");
        let duration = 5;
        self.run(duration);
        *self = Phase::Work(cycle);
        println!("{self:?}");
        self.check()
    }

    fn long_break(&mut self) {
        let mut cycle = 0;
        if let Phase::Work(n) = self {
            cycle = *n + 1
        }
        let duration = 30;
        self.run(duration);
        *self = Phase::Work(cycle);
        self.check()
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
    tomatui.check();
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
