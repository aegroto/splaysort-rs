#![feature(test)]

extern crate log;

mod splay;
mod sort;

use std::env;

use log::{warn, error};

use env_logger::Env;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        warn!("USAGE: {} input_size samples_count", args[0]);
        return;
    }

    sort::run_experiments(
        match args[1].parse::<usize>() {
            Ok(input_size) => input_size,
            Err(e) => {
                error!("Unable to parse input size '{}': {:?}", args[1], e);
                return;
            },
        }, 
        match args[2].parse::<u32>() {
            Ok(samples_count) => samples_count,
            Err(e) => {
                error!("Unable to parse samples count '{}': {:?}", args[2], e);
                return;
            },
        }
    );
}
