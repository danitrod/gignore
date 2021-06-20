use crate::{services::github::GithubService, utils::print_colored, Executable};
use async_trait::async_trait;
use clap::Clap;
use termcolor::Color;

pub const DESCRIPTION: &str =
    "List the available languages and technologies for gitignore templates";

#[derive(Clap)]
#[clap(about = DESCRIPTION)]
pub struct List {}

#[async_trait]
impl Executable for List {
    async fn exec(self) -> () {
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
