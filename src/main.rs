use tomatui::cli;
use tomatui::config::Config;
use tomatui::timer::Pomo;

fn main() {
    let args = cli::cli_args();
    cli::cli_confirmation(args);

    let config = Config::new();
    let mut tomatui = Pomo::new(config);
    tomatui.check(0);
}
