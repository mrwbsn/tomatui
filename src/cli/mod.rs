use std::io;

use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of task
    task: Option<String>,
}

pub fn cli_args() -> String {
    let cli = Cli::parse();
    let t;
    if let Some(task) = cli.task.as_deref() {
        t = task
    } else {
        t = "task"
    }

    t.to_owned()
}

pub fn cli_confirmation(args: String) {
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
