use clap::Parser;
#[derive(Parser, Debug)]
#[command(author = "Zhe Zhang")]
#[command(version)]
#[command(about = "cli tools")]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,

    /// Interval Sleep Print
    #[arg(short='s',long)]
    pub sleep: Option<u8>
}
