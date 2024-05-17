use crate::cli::BakeCli;
use anyhow::Result;
use handlebars::Handlebars;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

mod filesystem;
use filesystem::create_filesystem;

pub fn bake_sysext(cli: &BakeCli) -> Result<()> {
    let sys_ext_dir = create_extension_directory(&cli.name);
    create_extension_release_file(&cli, &sys_ext_dir);
    create_filesystem(&cli).unwrap();
    Ok(())
}

fn create_extension_directory(name: &str) -> PathBuf {
    let sys_ext_dir = format!("{}/usr/lib/extension-release.d", name);
    let sys_ext_dir_path = std::path::Path::new(&sys_ext_dir);
    fs::create_dir_all(sys_ext_dir_path).expect("Failed to create the sysext directory");
    sys_ext_dir_path.to_path_buf()
}

fn create_extension_release_file(cli: &BakeCli, sys_ext_dir: &PathBuf) {
    // render handlebars template for extension-release
    let template = embed_file::embed_string!("../templates/extension-release.hbs");
    let handlebars = Handlebars::new();
    let rendered = handlebars.render_template(&template, &cli).unwrap();
    let filename = sys_ext_dir.join(format!("extension-release.{}", cli.name));
    println!("Created extension-release file at: {}", &filename.display());
    File::create(filename)
        .unwrap()
        .write_all(rendered.as_bytes())
        .unwrap();
}
