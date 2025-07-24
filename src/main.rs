use clap::Parser;
use std::{
    env,
    fs::{self, write},
    io::{self},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Parser)]
#[command(version)]
struct Cli {}

#[derive(Debug, Error)]
enum Error {
    #[error("The env var `{name}` is missing.")]
    MissingEnvVar { source: env::VarError, name: String },

    #[error("failed to open event file : `{path}`")]
    OpenEventFile { source: io::Error, path: PathBuf },
}

fn env_var_load(env_name: &str) -> Result<String, Error> {
    env::var(env_name).map_err(|source| Error::MissingEnvVar {
        source,
        name: env_name.into(),
    })
}

fn main() -> anyhow::Result<()> {
    let _args = Cli::parse();
    let debug = env::var("ACTIONS_RUNNER_DEBUG").is_ok();
    let github_event_name = env_var_load("GITHUB_EVENT_NAME")?;

    if debug {
        eprintln!("::warning title=empty action::This action have been triggered by `{github_event_name}` but do nothing");
    }

    let github_event_path = env_var_load("GITHUB_EVENT_PATH")?;
    if debug {
        eprintln!("GITHUB_EVENT_PATH={github_event_path}");
    }

    let event = fs::read_to_string(&github_event_path).map_err(|source| Error::OpenEventFile {
        source,
        path: github_event_path.into(),
    })?;
    //if debug {
    eprintln!("event={event}");
    //}

    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let commits = "wip"; //TODO
    write(github_output_path, format!("commits={commits}")).unwrap();

    Ok(())
}
