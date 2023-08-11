use clap::{Args, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author = "Zhe Zhang")]
#[command(version)]
#[command(about = "cli tools")]
pub struct CliArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,

    /// Interval Sleep Print
    #[arg(short = 's', long)]
    pub sleep: Option<u8>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct SleepArgs {
    pub time: Option<u8>,

    #[arg(short = 'M', long)]
    pub minute: bool,

    #[arg(short = 'H', long)]
    pub hour: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// start sleep
    Sleep(SleepArgs),
}
