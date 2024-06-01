use clap::{CommandFactory, Error};

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
