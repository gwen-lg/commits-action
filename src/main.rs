use clap::Parser;
use std::env;

#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    let _args = Cli::parse();

    let github_event = env::var("GITHUB_EVENT").unwrap();

    eprintln!("::warning title=empty action::This action have been triggered by `{github_event}` but do nothing");
}
