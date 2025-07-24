use clap::Parser;
use std::{env, fs::write};

#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    let _args = Cli::parse();

    let github_event = env::var("GITHUB_EVENT").unwrap();

    eprintln!("::warning title=empty action::This action have been triggered by `{github_event}` but do nothing");

    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let commits = "wip"; //TODO
    write(github_output_path, format!("commits={commits}")).unwrap();
}
