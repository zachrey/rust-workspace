use clap::Parser;
use futures::executor::block_on;
use super::{args::Args, interval};

pub fn create_cli_instance() {
    let args = Args::parse();

    match args.name {
        Some(name) => {
            for _ in 0..args.count {
                println!("Hello {}!", name);
            }
        },
        None => {}
    }

    match args.sleep {
        Some(times) => {
            let op = interval::print::interval_print(times);
            block_on(op);
        },
        None => {}
    }
}
