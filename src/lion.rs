//! Contains items related to main functionality of lion.

use crate::lion_config::Config;
use crate::prelude::*;
use cfg_if::cfg_if;
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

        let src_dir = Path::join(Path::new(&proj_name), "src");

        // Create "src" folder
        if !fs::exists(&src_dir)? {
            fs::create_dir(&src_dir).context("creating 'src' dir in lion project")?;
        } else {
            return Err(anyhow!("folder with name {src_dir:?} already exists"))
        }

        // Put basic "main" file into "src" folder
        fs::write(Path::join(&src_dir, "main.cpp"), r#"
        #include <stdio.h>
        int main() { printf("%d\n", 23); }
        "#)
            .context("generating basic 'src/main.c' file")?;

        // Generate basic config file in newly created folder
        fs::write(Path::join(Path::new(&proj_name), "lion.toml"), format!(r#"
[pkg]
name = "{proj_name}"
description = ""

[bin]
files = ["src/main.cpp"]
out_dir = "dist"
out_file = "out.exe"
        "#).trim_start()).context("writing new simple lion config")?;

        Ok(())
    }

    /// Parses the given config file path to the `Config` struct.
    pub fn parse_config(p: &str) -> anyhow::Result<Config> {
        Ok(fs::read_to_string(p).context("reading lion toml config file")?.into())
    }

    /// Compiles all files in the current project.
    pub fn compile(&self) -> anyhow::Result<()> {
        #[allow(unused_mut)]
        let mut files = self.config.bin.files.clone();

        // Add 'test/' folder prefix when running this function from a test
        cfg_if!{
            if #[cfg(test)] {
                files = files.into_iter().map(|i| String::from("test/") + &i).collect();
            }
        }

        #[allow(unused_mut)]
        // Get out_dir from config, or if none, make it "dist"
        let mut out_dir = self.config.bin.out_dir
            .to_owned()
            .unwrap_or("dist".to_string());

        // Prepend with 'test/' folder during testing
        cfg_if!{
            if #[cfg(test)] {
                out_dir = String::from("test/") + out_dir.as_str();
            }
        }

        // Convert out_dir to a `&Path`
        let out_dir = Path::new(
            &out_dir
        );

        // Construct out_dir if it doesn't already exist
        if !fs::exists(out_dir).context(format!("checking if dir '{out_dir:?}' exists"))? {
            fs::create_dir(out_dir).context(format!("creating '{out_dir:?}' out dir for binary"))?;
        }

        let out_file_name = &self.config.bin.out_file;

        // Compile project files with clang
        let compile_output = Command::new("clang++")
            .args(files)
            // Specifies output file path
            .args([
                "-o",
                out_dir
                .join(
                out_file_name
                        .to_owned()
                        .unwrap_or("out.exe".to_string()).as_str()
                ).to_str().expect("failed to construct output path for -o clang++ arg")
            ])
            .output()?;

        // Return error if clang compilation failed
        if !compile_output.stderr.is_empty() {
            return Err(anyhow!("clang compilation error: {}", String::from_utf8(compile_output.stderr)?));
        }

        Ok(())
    }

    #[allow(dead_code)]
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

    #[test]
    fn test_compile_project() {
        // Create test project
        Lion::create_new_project("test".to_string()).context("creating new test lion project").unwrap();
        
        // Compile project
        Lion::new(Lion::parse_config("test/lion.toml").context("parsing lion.toml").unwrap())
            .compile()
            .context("testing project compilation")
            .unwrap();

        // Delete created test project
        fs::remove_dir_all("test").context("deleting test lion project dir").unwrap();
    }
}
