use clap::Parser;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
#[command(version, about, long_about = None)]
pub struct RenderCli {
    #[arg(short, long)]
    pub template_file: Option<String>,
    #[arg(short, long)]
    pub output_file: Option<String>,
    /// Values to render the template in the form of key=value
    #[arg(required = false)]
    pub values: Vec<String>,
}