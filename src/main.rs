mod cli;
mod core;
mod tui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Set { file, monitor }) => {
            cli::commands::cmd_set(&file, monitor.as_deref())
        }
        Some(Commands::Stop { monitor }) => {
            cli::commands::cmd_stop(monitor.as_deref())
        }
        Some(Commands::Enable) => cli::commands::cmd_enable(),
        Some(Commands::Disable) => cli::commands::cmd_disable(),
        Some(Commands::Status) => cli::commands::cmd_status(),
        Some(Commands::List) => cli::commands::cmd_list(),
        None => {
            // No subcommand — launch TUI
            tui::run()
        }
    };

    if let Err(e) = result {
        eprintln!("\x1b[31merror:\x1b[0m {}", e);
        std::process::exit(1);
    }
}
