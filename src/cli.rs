use clap::{ArgAction, Parser, ValueEnum};
use serde::Serialize;

#[derive(Parser, Serialize)]
#[command(version, about, long_about = None)]
pub struct BakeCli {
    /// Name of the sys-ext to build.
    #[arg(index = 1, required = true)]
    pub name: String,
    /// To build for a specific OS, pass the ID as found under 'etc/os-release' or '_any' for any OS
    #[arg(short, long, default_value = "_any")]
    pub os: String,
    /// Filesystem to create for the sys-ext
    #[arg(value_enum, long, short, default_value = "squashfs")]
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
