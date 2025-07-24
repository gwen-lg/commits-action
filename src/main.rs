use std::env;

fn main() {
    let github_event = env::var("GITHUB_EVENT").unwrap();

    eprintln!("::warning title=empty action::This action have been triggered by `{github_event}` but do nothing");
}
