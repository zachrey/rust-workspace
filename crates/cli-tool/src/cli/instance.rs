use super::{
    args::{CliArgs, Commands},
    interval,
};
use clap::Parser;
use futures::executor::block_on;

pub fn create_cli_instance() {
    let args = CliArgs::parse();

    match args.name {
        Some(name) => {
            for _ in 0..args.count {
                println!("Hello {}!", name);
            }
        }
        None => {}
    }

    match args.sleep {
        Some(times) => {
            let op = interval::print::interval_print(times);
            block_on(op);
        }
        None => {}
    }

    match &args.command {
        Commands::Sleep(sleep_args) => {
            block_on(interval::print::interval_print_v2(sleep_args));
        }
    }
}
