use clap::Parser;
use linya::Progress;
use std::io;

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



    let mut ks = String::new();
    println!("Do you want to start [{t}] with Tomatui? [Y/n]");

    while ks != "Y" && ks != "y" {
        ks.clear();
        io::stdin().read_line(&mut ks).unwrap();
        ks = ks.trim().to_owned();

        if ks == "" {
            break;
        }

        println!("{ks:?}");
    }

    run(t.to_owned())
}

fn run(t: String) {
    let mut progress = Progress::new();
    println!("{t}");
    let bar = progress.bar(50, "Work!");

    progress.set_and_draw(&bar, 50);
}
