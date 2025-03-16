//! Contains items related to main functionality of lion.

use crate::lion_config::Config;
use crate::prelude::*;
use crate::args::Args;
use std::process::Command;
use std::fs;
use std::path::Path;

/// Project manager that can add dependencies, run the project, and so much more.
#[derive(Clone, Debug, PartialEq)]
pub struct Lion {
    config: Config
}

impl Lion {
    /// Constructs a new Lion instance.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Creates a new folder with essential components for lion project.
    pub fn create_new_project(proj_name: String) -> anyhow::Result<()> {
        // Create folder with name of project
        if !fs::exists(&proj_name)? {
            fs::create_dir(&proj_name).context("creating new lion project dir")?;
        } else {
            return Err(anyhow!("folder with name {proj_name} already exists"))
        }

        // Generate basic config file in newly created folder
        fs::write(Path::join(&Path::new(&proj_name), "lion.toml"), format!(r#"
            [pkg]
            name = "{proj_name}"
            description = ""

            [bin]
            files = ["src/main.c"]
            out = "dist/out.exe"
        "#)).context("writing new simple lion config")?;

        Ok(())
    }

    /// Compiles all files in the current project.
    pub fn compile(&self) -> anyhow::Result<()> {
        let files = &self.config.bin.files;
        let out_file_name = &self.config.bin.out;

        // Compile project files with clang
        Command::new("clang++")
            .args(files)
            // Specifies output file path
            .args([
                "-o",
                out_file_name
                    .clone()
                    .unwrap_or("out.exe".to_string()).as_str()
            ])
            .output()?;

        todo!()
    }

    /// Compiles and runs the current project.
    pub fn run(&self) -> anyhow::Result<()> {
        self.compile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project() {
        // Create test project
        Lion::create_new_project("test".to_string()).context("creating new test lion project").unwrap();

        // Delete created test project
        fs::remove_dir_all("test").context("deleting test lion project dir").unwrap();
    }
}
