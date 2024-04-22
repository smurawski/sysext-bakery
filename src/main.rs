use clap::{Parser, ValueEnum, ArgAction};
use handlebars::Handlebars;
use serde::Serialize;
use std::{fs::{self, File}, io::Write, path::PathBuf};

#[derive(Parser, Serialize)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the sys-ext to build. 
    #[arg(index=1, required=true)]
    name: String,
    /// To build for a specific OS, pass the ID as found under 'etc/os-release' or '_any' for any OS
    #[arg(short, long, default_value="_any")]
    os: String,
    /// Filesystem to create for the sys-ext
    #[arg(value_enum, long, short, default_value="squashfs")]
    format: FileSystemFormat,
    /// If arch is specified the sysext image will be required to run on the given architecture.
    #[arg(value_enum, short, long)]
    arch: Option<Arch>,
    #[arg(short, long, action = ArgAction::SetTrue)]
    reload: bool,
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Arch {
    Amd64,
    X86_64,
    Aarch64,
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum FileSystemFormat {
    Btrfs,
    Ext4,
    Ext2,
    Squashfs,
}

fn create_extension_directory(name: &str) -> PathBuf{
    let sys_ext_dir = format!("{}/usr/lib/extension-release.d", name);
    let sys_ext_dir_path = std::path::Path::new(&sys_ext_dir);
    fs::create_dir_all(sys_ext_dir_path).expect("Failed to create the sysext directory");
    sys_ext_dir_path.to_path_buf()
}

fn create_extension_release_file(cli: &Cli, sys_ext_dir: &PathBuf) {
    // render handlebars template for extension-release
    let template = embed_file::embed_string!("templates/extension-release.hbs");
    let handlebars = Handlebars::new();
    let rendered = handlebars.render_template(&template, &cli).unwrap();
    let filename = sys_ext_dir.join(format!("extension-release.{}", cli.name));
    println!("Created extension-release file at: {}", &filename.display());
    File::create(filename).unwrap().write_all(rendered.as_bytes()).unwrap();
}


fn main() {
    let cli = Cli::parse();

    let sys_ext_dir = create_extension_directory(&cli.name);
    create_extension_release_file(&cli, &sys_ext_dir);
    // TODO: create the filesystem
}
