use std::fs;

use crate::{
    services::github::GithubService,
    utils::{get_language_aliases, print_colored},
};
use termcolor::Color;

use super::Executable;
use async_trait::async_trait;
use clap::Clap;

pub const DESCRIPTION: &str = "Create an up to date gitignore for a given language or technology";

#[derive(Clap)]
#[clap(about = DESCRIPTION)]
pub struct Create {
    language: String,
}

#[async_trait]
impl Executable for Create {
    async fn exec(self) -> () {
        let github_service = GithubService::new();

        // The raw github page is case-sensitive
        // To match uncased args, we need to check what is the cased name
        let aliases = get_language_aliases();
        let mut actual_lang = self.language.to_lowercase();
        if let Some(lang) = aliases.get(&actual_lang) {
            actual_lang = lang.clone();
        }

        let list = github_service.list_languages().await;
        let mut cased_language = None;
        for lang in list {
            if lang.to_lowercase() == actual_lang {
                cased_language = Some(lang);
            }
        }

        match cased_language {
            None => print_colored(
                "Not found: this language or technology has no available gitignore template."
                    .into(),
                Color::Red,
            ),
            Some(lang) => {
                let contents = github_service.get_gitignore(lang.clone()).await;
                fs::write(".gitignore", contents).unwrap();
                print_colored(
                    format!(".gitignore file for {} successfully created", lang),
                    Color::Green,
                );
            }
        }
    }
}
