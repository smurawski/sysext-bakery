use anyhow::Result;
use crate::{BakeCli, FileSystemFormat, commands::Command};



pub fn create_filesystem(cli: &BakeCli) -> Result<()>{
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
