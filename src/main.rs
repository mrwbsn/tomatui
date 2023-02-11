use std::io;
use clap::Parser;
use linya::Progress;

// enum Phase {
//     Work,
//     ShortBreak,
//     LongBreak
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of task
    task: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let t;
    if let Some(task) = cli.task.as_deref() {
        t = task
    } else {
        t = "task"
    }

    println!("Do you want to start [{t}] with Tomatui? [Y/n]");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("{input}");




}

fn run(t: String) {
        let mut progress = Progress::new();
        let bar = progress.bar(50, t);

        progress.set_and_draw(&bar, 10);
    
}
