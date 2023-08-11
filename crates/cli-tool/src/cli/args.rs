use clap::{Args, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author = "Zhe Zhang")]
#[command(version)]
#[command(about = "cli tools")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// start sleep
    Sleep(SleepArgs),
    /// url request
    Request(RequestArgs),
}

#[derive(Args, Debug)]
pub struct SleepArgs {
    pub time: Option<u8>,

    #[arg(short = 'M', long)]
    pub minute: bool,

    #[arg(short = 'H', long)]
    pub hour: bool,
}

#[derive(Args, Debug)]
pub struct RequestArgs {
    /// request url string
    pub url: String,

    /// request method
    #[arg(short = 'm', long, default_value_t = String::from("Get"))]
    pub method: String,

    #[arg(short = 'd', long)]
    pub data: Option<String>,
}
