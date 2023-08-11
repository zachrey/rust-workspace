use crate::cli::request::router::create_request;

use super::{
    args::{CliArgs, Commands},
    interval,
};
use clap::Parser;
use futures::executor::block_on;

pub fn create_cli_instance() {
    let args = CliArgs::parse();

    match &args.command {
        Commands::Sleep(sleep_args) => {
            block_on(interval::print::interval_print_v2(sleep_args));
        }
        Commands::Request(request_args) => {
            block_on(create_request(request_args));
        }
    }
}
