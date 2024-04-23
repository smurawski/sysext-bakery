mod find_command;
use find_command::find_command;

use anyhow::Result;
use duct::{cmd, ReaderHandle};
use log::{debug, trace};
use std::{path::PathBuf, process::ExitStatus};

#[derive(Clone, Debug)]
pub struct Command<'a> {
    name: String,
    subcommand: Option<String>,
    path: PathBuf,
    args: Vec<&'a str>,
    stdout: Option<String>,
    stderr: Option<String>,
    exit_status: Option<ExitStatus>,
    verbose: bool,
    show_progress: bool,
    working_directory: Option<PathBuf>,
}

impl<'a> Command<'a> {
    pub fn new(name: &str) -> Self {
        let path = find_command(name).unwrap();
        Command {
            name: name.to_owned(),
            subcommand: None,
            path,
            args: Vec::new(),
            stdout: None,
            stderr: None,
            exit_status: None,
            verbose: false,
            show_progress: false,
            working_directory: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    #[allow(dead_code)]
    pub fn with_subcommand(mut self, subcommand: &str) -> Self {
        self.subcommand = Some(subcommand.to_owned());
        self
    }

    #[allow(dead_code)]
    pub fn with_args(mut self, args: Vec<&'a str>) -> Self {
        self.args = args;
        self
    }

    #[allow(dead_code)]
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    #[allow(dead_code)]
    pub fn with_working_directory(mut self, working_directory: &PathBuf) -> Self {
        self.working_directory = Some(working_directory.clone());
        self
    }

    #[allow(dead_code)]
    pub fn with_show_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }

    #[allow(dead_code)]
    pub fn get_stdout(&self) -> Option<String> {
        self.stdout.clone()
    }

    #[allow(dead_code)]
    pub fn get_stderr(&self) -> Option<String> {
        self.stderr.clone()
    }

    #[allow(dead_code)]
    pub fn success(&self) -> bool {
        if let Some(s) = &self.exit_status {
            s.success()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn stdout_reader(&self) -> Result<ReaderHandle> {
        // trace!("Command: {} {} running", &self.name, &self.subcommand);
        // debug!("\t`{} {} {}`", &self.name, &self.subcommand, &self.args.join(" "));
        let mut command_args: Vec<&str> = Vec::new();
        if let Some(subcommand) = &self.subcommand {
            command_args.push(subcommand)
        };
        for arg in &self.args {
            command_args.push(arg);
        }
        let reader = cmd(&self.path, command_args).stderr_capture().reader()?;
        trace!("Returning reader handle.");
        Ok(reader)
    }

    #[allow(dead_code)]
    pub fn stderr_reader(&self) -> Result<ReaderHandle> {
        // trace!("Command: {} {} running", &self.name, &self.subcommand);
        // debug!("\t`{} {} {}`", &self.name, &self.subcommand, &self.args.join(" "));
        let mut command_args: Vec<&str> = Vec::new();
        if let Some(subcommand) = &self.subcommand {
            command_args.push(subcommand)
        };
        for arg in &self.args {
            command_args.push(arg);
        }
        let reader = cmd(&self.path, command_args).stdout_capture().reader()?;
        trace!("Returning reader handle.");
        Ok(reader)
    }

    #[allow(dead_code)]
    pub fn run(mut self) -> Result<Self> {
        // trace!("Command: {} {} running", &self.name, &self.subcommand);
        // debug!("\t`{} {} {}`", &self.name, &self.subcommand, &self.args.join(" "));
        let mut command_args: Vec<&str> = Vec::new();
        if let Some(subcommand) = &self.subcommand {
            command_args.push(subcommand)
        };
        for arg in &self.args {
            command_args.push(*arg);
        }
        let mut command = cmd(&self.path, command_args);
        if let Some(working_directory) = &self.working_directory {
            command = command.dir(working_directory);
        }
        let output = command
            .stderr_capture()
            .stdout_capture()
            .unchecked()
            .run()?;
        self.stdout = Some(String::from_utf8(output.stdout)?);
        self.stderr = Some(String::from_utf8(output.stderr)?);
        self.exit_status = Some(output.status);
        debug!("  Command stdout: {:?}", &self.stdout);
        debug!("  Command stderr: {:?}", &self.stderr);
        // trace!("Finished with command {} {}", &self.name, &self.subcommand);

        Ok(self)
    }
}
