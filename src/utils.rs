use std::collections::HashMap;
use std::path::Path;

use clap::{CommandFactory, Error};
use serde::de::DeserializeOwned;

use crate::Cli;

pub fn parse_addition(s: &str) -> Result<(String, String), &'static str> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or("--addition must have format k=v")
}

pub fn unwrap_or_missing_argument<T>(
    option: Option<T>,
    key: &str,
    cause: Option<&str>,
) -> Result<T, Error> {
    option.ok_or_else(|| {
        Cli::command().error(
            clap::error::ErrorKind::MissingRequiredArgument,
            format!("--{} is required if {}", key, cause.unwrap_or("--config is not used")),
        )
    })
}

pub fn load_predefined_and_local<T: DeserializeOwned>(
    path: impl AsRef<Path>,
) -> HashMap<String, T> {
    let config = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let local_path = path.as_ref().with_file_name(".local.toml");
    if !local_path.exists() {
        config
    } else {
        config
            .into_iter()
            .chain(
                toml::from_str::<HashMap<_, _>>(&std::fs::read_to_string(local_path).unwrap())
                    .unwrap(),
            )
            .collect()
    }
}

#[cfg(test)]
pub fn get_dot_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
