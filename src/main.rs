mod display_shortcodes;
mod event;
mod log;
mod state;

use log::log;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct AustrianOak {
    #[clap(subcommand)]
    command: Command,
}

// TODO
// 1. Display last n days (default 1) of data for a given event

#[derive(Subcommand)]
enum Command {
    Log { input: String },
    Shortcodes,
    // TODO: Add Show command: `show dl` gives the last deadlift day
}

fn main() {
    let args = AustrianOak::parse();
    let result = match args.command {
        Command::Log { input } => log(input),
        Command::Shortcodes => display_shortcodes(),
    };
    if let Err(e) = result {
        println!("{}", e);
    }
}
