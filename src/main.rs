use crate::bake::bake_sysext;
use clap::Parser;
use env_logger::Env;

mod bake;
mod cli;
mod native_commands;

use cli::BakeCli;

fn main() {
    let cli = BakeCli::parse();
    env_logger::init_from_env(Env::default().default_filter_or("error"));

    bake_sysext(&cli).unwrap();
}
