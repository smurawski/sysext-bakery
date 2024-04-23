use anyhow::Result;
use crate::{BakeCli, FileSystemFormat, commands::Command};
use handlebars::Handlebars;
use std::{fs::{self, File}, io::Write, path::PathBuf, vec};


pub fn bake_sysext(cli: &BakeCli) -> Result<()> {
    let sys_ext_dir = create_extension_directory(&cli.name);
    create_extension_release_file(&cli, &sys_ext_dir);
    create_filesystem(&cli).unwrap();
    Ok(())
}

fn create_extension_directory(name: &str) -> PathBuf{
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
    File::create(filename).unwrap().write_all(rendered.as_bytes()).unwrap();
}

fn create_filesystem(cli: &BakeCli) -> Result<()>{
    match cli.format {
        FileSystemFormat::Ext4 => create_filesystem_ext4(cli)?,
        FileSystemFormat::Ext2 => create_filesystem_ext2(cli)?,
        FileSystemFormat::Btrfs => create_filesystem_btrfs(cli)?,
        FileSystemFormat::Squashfs => create_filesystem_squashfs(cli)?,
    };
    Ok(())
}

fn create_filesystem_ext4(cli: &BakeCli) -> Result<()> {
    let raw_file_name = format!("{}.raw", &cli.name);
    Command::new("truncate")
        .with_args(vec!["-s", "1G", &raw_file_name]) 
        .run()?;
    Command::new("mkfs.ext4")
        .with_args(vec!["-E", "root_owner=0:0", "-d", &cli.name, &raw_file_name])
        .run()?;
    Command::new("resize2fs")
        .with_args(vec!["-M", &raw_file_name])
        .run()?;
    Ok(())
}

fn create_filesystem_ext2(cli: &BakeCli) -> Result<()> {
    let raw_file_name = format!("{}.raw", &cli.name);
    Command::new("truncate")
        .with_args(vec!["-s", "1G", &raw_file_name])
        .run()?;
    Command::new("mkfs.ext2")
        .with_args(vec!["-E", "root_owner=0:0", "-d", &cli.name, &raw_file_name])
        .run()?;
    Command::new("resize2fs")
        .with_args(vec!["-M", &raw_file_name])
        .run()?;
    Ok(())
}

fn create_filesystem_btrfs(cli: &BakeCli) -> Result<()> {
    let raw_file_name = format!("{}.raw", &cli.name);
    Command::new("mkfs.btrfs")
        .with_args(vec!["--mixed", "-m", "single", "-d", "single", "--shrink", "--rootdir", &cli.name, &raw_file_name])
        .run()?;
    Ok(())
}

fn makesquashfs_version_gt_4_6() -> Result<bool> {
    let mksquashfs_cmd = Command::new("mksquashfs")
    .with_args(vec!["-version"]).run()?;
    if let Some(output) = mksquashfs_cmd.get_stdout() {
        if let Some(version_line) = output.lines().next() {
            let version_line= version_line.split_whitespace().collect::<Vec<&str>>();
            let versions = version_line[2].split('.').collect::<Vec<&str>>();
            let major = versions[0].parse::<u32>().unwrap();
            let minor = versions[1].parse::<u32>().unwrap();
            if major > 4 || (major == 4 && minor >= 6) {
                return Ok(true);
            }
        };
    };
    Ok(false)
}

fn create_filesystem_squashfs(cli: &BakeCli) -> Result<()>{
    let raw_file_name = format!("{}.raw", &cli.name);
    let mut args = vec![&cli.name, &raw_file_name, "-all-root", "-noappend"];
    if makesquashfs_version_gt_4_6()? {
        args.push("-xattrs-exclude' '^btrfs.");
    }
    Command::new("mksquashfs")
        .with_args(args)
        .run()?;
    Ok(())
}
