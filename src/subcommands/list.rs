use crate::{services::github::GithubService, utils::print_colored, Executable};
use async_trait::async_trait;
use clap::Parser;
use termcolor::Color;

pub const DESCRIPTION: &str =
    "List the available languages and technologies for gitignore templates";

#[derive(Parser)]
#[clap(about = DESCRIPTION)]
pub struct List {}

#[async_trait]
impl Executable for List {
    async fn exec(self) -> () {
        // TODO: Cache the list locally and attempt to update it each time the command runs
        let github_service = GithubService::new();
        let files = github_service.list_languages().await;

        print_colored(
            "Available languages and technologies:\n".into(),
            Color::Green,
        );
        for file in files {
            println!("{}", file);
        }
        println!();
    }
}
