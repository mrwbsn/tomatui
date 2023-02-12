use clap::Parser;
use linya::Progress;
use std::io;

enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of task
    task: Option<String>,
}

impl Cli {
    fn run(&self) {
        let tomato_phase = Phase::Work;
        match tomato_phase {
            Phase::Work => &self.work(),
            Phase::ShortBreak => println!("short"),
            Phase::LongBreak => println!("long"),
        }
    }
    fn work(&self) {
        let mut progress = Progress::new();
        let bar = progress.bar(50, "Work!");

        progress.inc_and_draw(&bar, 50);
    }
}

fn main() {
    let cli = Cli::parse();
    let t;
    if let Some(task) = cli.task.as_deref() {
        t = task
    } else {
        t = "task"
    }

    let mut ks = String::new();
    println!("Do you want to start [{t}] with Tomatui? [Y/n]");

    while ks != "Y" && ks != "y" {
        ks.clear();
        io::stdin().read_line(&mut ks).unwrap();
        ks = ks.trim().to_owned();

        if ks == "" {
            break;
        }
    }

    Cli::run();
}
