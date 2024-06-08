use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Start Pomodoro Session
    Start {},
    /// Configure your costum Session
    Config(ConfigArgs),
    /// Get current Session State
    Update,
    /// Stopping Pomodoro Session
    Stop,
    /// Sketchybar Interface
    Sketchybar(SketchybarArgs),
}

#[derive(Args)]
pub(crate) struct ConfigArgs {
    pub(crate) work_duration: Option<u64>,
    pub(crate) break_short: Option<u64>,
    pub(crate) break_long: Option<u64>,
}

#[derive(Args)]
pub(crate) struct SketchybarArgs {
    #[command(subcommand)]
    pub(crate) commands: SketchybarCommand,
}

#[derive(Subcommand)]
pub(crate) enum SketchybarCommand {
    Load { icon: Option<char> },
    Unload,
    Update,
    Show,
    Hide,
}
