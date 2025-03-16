//! Items related to handling the `lion.toml` file in lion projects.

use crate::prelude::*;
use serde::{Serialize, Deserialize};

/// Structure representing data of `lion.toml` file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub pkg: Pkg,
    pub bin: Bin
}

impl From<&str> for Config {
    fn from(val: &str) -> Config {
        toml::from_str(val).context("Calling into() to turn `String` into `Config`.").unwrap()
    }
}

impl From<String> for Config {
    fn from(val: String) -> Config {
        toml::from_str(val.as_str()).context("Calling into() to turn `&str`into `Config`.").unwrap()
    }
}

/// TOML Table holding metadata about current library/package.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pkg {
    pub name: String,
    pub description: Option<String>,
    pub long_description: Option<String>
}

/// TOML Table holding metadata about the project files.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bin {
    /// Files in project to compile.
    pub files: Vec<String>,
    /// Output dir.
    pub out_dir: Option<String>,
    /// Filename of compiled .exe file.
    pub out_file: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        // Create str contents of basic lion config file
        let toml_config_str = r#"
            [pkg]
            name = "Mike"
            description = "Mikee"

            [bin]
            files = ["src/main.c"]
        "#;

        // Ensure that str contents of config were parsed correctly into `Config`
        assert_eq!(
            toml::from_str::<Config>(toml_config_str)
            .context("testing deserialization of lion toml config")
            .unwrap(),
            Config{
                pkg: Pkg {
                    name: "Mike".to_string(),
                    description: Some("Mikee".to_string()),
                    long_description: None
                },
                bin: Bin {
                    files: vec!["src/main.c".to_string()],
                    out_dir: None,
                    out_file: None
                }
            }
        );
    }
}
