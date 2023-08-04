use clap::Parser;

use super::args::Args;

pub fn create_cli_instance() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
