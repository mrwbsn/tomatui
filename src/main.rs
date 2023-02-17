use tomatui::cli;
use tomatui::config::Config;
use tomatui::timer::Tomato;

fn main() {
    let args = cli::cli_args();
    cli::cli_confirmation(args);

    let config = Config::new();
    let mut tomatui = Tomato::new(config);
    tomatui.check(0);
}
