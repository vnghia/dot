use clap::CommandFactory;

use crate::Cli;

pub fn parse_addition(s: &str) -> Result<(String, String), &'static str> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or("--addition must have format k=v")
}

pub fn unwrap_or_missing_argument<T>(option: Option<T>, key: &str) -> T {
    if let Some(value) = option {
        value
    } else {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                format!("--{} is required if --config is not used", key),
            )
            .exit()
    }
}
