use super::config;
use clap::Clap;

pub mod cat;
pub mod create;
pub mod list;

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL)]
    Cat(cat::Cat),

    #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL)]
    Create(create::Create),

    #[clap(version = config::VERSION, author = config::AUTHOR_EMAIL)]
    List(list::List),
}

pub trait Executable {
    /// Executes the actual logic of the command
    fn exec(self) -> ();
}
