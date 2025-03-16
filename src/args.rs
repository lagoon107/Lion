use clap::{Parser, Subcommand};

/// The main app args.
#[derive(Clone, Debug, Parser, PartialEq)]
pub struct Args {
    #[command(subcommand)]
    pub command: RootCommand
}

/// The main (root) command.
#[derive(Clone, Debug, Subcommand, PartialEq)]
pub enum RootCommand {
    /// Initializes new project with basic config file.
    New{
        name: String
    },
    /// Compiles project in current working directory.
    Build
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_arg_new() {
        // Create sample args
        let sample_args = vec!["", "new", "sample_name"];
        // Parse sample args
        let parsed_args = Args::try_parse_from(sample_args.clone().into_iter())
            .context(format!("parsing {sample_args:?} into `Args` data structure"))
            .unwrap();

        // Assert parsed args have correct info
        assert_eq!(parsed_args, Args{
            command: RootCommand::New{
                name: "sample_name".to_string()
            }
        });
    }

    #[test]
    fn test_arg_build() {
        // Create sample args
        let sample_args = vec!["", "build"];
        // Parse sample args
        let parsed_args = Args::try_parse_from(sample_args.clone().into_iter())
            .context(format!("parsing {sample_args:?} into `Args` data structure"))
            .unwrap();

        // Assert parsed args have correct info
        assert_eq!(parsed_args, Args{
            command: RootCommand::Build
        });
    }
}
