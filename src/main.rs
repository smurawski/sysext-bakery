use crate::bake::bake_sysext;
use crate::render::render_templates;
use clap::Parser;
use env_logger::Env;

mod bake;
mod render;
mod cli;
mod native_commands;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    env_logger::init_from_env(Env::default().default_filter_or("error"));

    if cli.test {
        run_test();
        return;
    }
    if let Some(cmd) = cli.cmd {
        match cmd {
            cli::Commands::Bake(bakery_cli) => bake_sysext(&bakery_cli).unwrap(),
            cli::Commands::Render(render_cli) => render_templates(&render_cli),
            
        }
    }
}

fn run_test() {}
