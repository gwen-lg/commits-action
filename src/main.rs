use clap::Parser;
use std::{env, fs::write};
use thiserror::Error;

#[derive(Parser)]
#[command(version)]
struct Cli {}

#[derive(Debug, Error)]
enum Error {
    #[error("The env var `GITHUB_EVENT_NAME` is missing.")]
    MissingGithubEventNameEnvVar(#[source] env::VarError),
}

fn main() -> anyhow::Result<()> {
    let _args = Cli::parse();
    let github_event_name =
        env::var("GITHUB_EVENT_NAME").map_err(Error::MissingGithubEventNameEnvVar)?;

    eprintln!("::warning title=empty action::This action have been triggered by `{github_event_name}` but do nothing");

    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let commits = "wip"; //TODO
    write(github_output_path, format!("commits={commits}")).unwrap();

    Ok(())
}
