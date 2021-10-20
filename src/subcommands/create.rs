use super::Executable;
use crate::{
    services::github::GithubService,
    utils::{get_language_aliases, print_colored},
};
use async_trait::async_trait;
use clap::Parser;
use std::{fs, path};
use termcolor::Color;

pub const DESCRIPTION: &str = "Create an up to date gitignore for a given language or technology";

#[derive(Parser)]
#[clap(about = DESCRIPTION)]
pub struct Create {
    #[clap(about = "Language or technology for the gitignore template")]
    language: String,
    #[clap(
        short,
        long,
        about = "A name for the file to be created",
        default_value = ".gitignore"
    )]
    name: String,
    #[clap(
        short,
        long,
        about = "Path where to create the file",
        default_value = "."
    )]
    path: String,
}

#[async_trait]
impl Executable for Create {
    async fn exec(self) -> () {
        // Validate inputs
        let path = path::Path::new(&self.path);
        if !path.is_dir() {
            print_colored(format!("Error: Path {} not found", self.path), Color::Red);
            std::process::exit(3);
        }

        let github_service = GithubService::new();

        // Resolve any aliases for languages
        let aliases = get_language_aliases();
        let mut lowercased_lang = self.language.to_lowercase();
        if let Some(lang) = aliases.get(&lowercased_lang) {
            lowercased_lang = lang.clone();
        }

        // Github page is case-sensitive
        // Get the cased name from the available list
        let list = github_service.list_languages().await;
        let mut cased_language = None;
        for lang in list {
            if lang.to_lowercase() == lowercased_lang {
                cased_language = Some(lang);
            }
        }

        match cased_language {
            None => print_colored(
                format!("Not found: the language or technology {} currently has no available gitignore template.", self.language),
                Color::Yellow,
            ),
            Some(lang) => {
                let contents = github_service.get_gitignore(lang.clone()).await;
                let path = path.join(self.name);

                match fs::write(path.clone(), contents) {
                    Ok(_) => print_colored(
                        format!(".gitignore file for {} successfully created at {}", lang, path.to_str().unwrap()),
                        Color::Green,
                    ),
                    Err(err) => {
                        print_colored(format!("Error: {}", err), Color::Red);
                    }
                };
            }
        }
    }
}
