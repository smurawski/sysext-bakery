use clap::{Parser, ValueEnum, ArgAction};
use env_logger::Env;

use serde::Serialize;
use lazy_static::lazy_static;


mod commands;
mod bake;
use bake::bake_sysext;

#[derive(Parser, Serialize)]
#[command(version, about, long_about = None)]
pub struct BakeCli {
    /// Name of the sys-ext to build. 
    #[arg(index=1, required=true)]
    pub name: String,
    /// To build for a specific OS, pass the ID as found under 'etc/os-release' or '_any' for any OS
    #[arg(short, long, default_value="_any")]
    pub os: String,
    /// Filesystem to create for the sys-ext
    #[arg(value_enum, long, short, default_value="squashfs")]
    pub format: FileSystemFormat,
    /// If arch is specified the sysext image will be required to run on the given architecture.
    #[arg(value_enum, short, long)]
    pub arch: Option<Arch>,
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub reload: bool,
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Arch {
    Amd64,
    X86_64,
    Aarch64,
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum FileSystemFormat {
    Btrfs,
    Ext4,
    Ext2,
    Squashfs,
}



lazy_static! {
    pub static ref VERSION: String = format!("v{}", env!("CARGO_PKG_VERSION"));
    //pub static ref VERBOSE: bool = get_app_cli(&VERSION).get_matches().is_present("verbose");
}

fn main() {
    
    let cli = BakeCli::parse();
    env_logger::init_from_env(
        Env::default().default_filter_or("error")
    );

    bake_sysext(&cli).unwrap();   
}
