use super::config;
use async_trait::async_trait;
use clap::Clap;

pub mod cat;
pub mod create;
pub mod list;

#[derive(Clap)]
pub enum SubCommand {
    // #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL, visible_alias = "show")]
    // Cat(cat::Cat),
    #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL,  visible_alias = "new")]
    Create(create::Create),

    #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL, visible_alias = "ls")]
    List(list::List),
}

#[async_trait]
pub trait Executable {
    /// Executes the actual logic of the command
    async fn exec(self) -> ();
}
