use clap::{ArgAction, Parser, Subcommand};

mod bake;
mod render;

pub use bake::{BakeCli, FileSystemFormat};
pub use render::RenderCli;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "bakery", version = "0.1.0")]
#[command(arg_required_else_help = true)]
#[command(about = "A tool for helping to bake sys-extensions", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
    #[arg(short, long, action = ArgAction::SetTrue, hide = true)]
    pub test: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Bake(BakeCli),
    Render(RenderCli),
}
