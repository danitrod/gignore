use clap::Parser;

mod config;
mod services;
mod subcommands;
mod utils;

use subcommands::{Executable, SubCommand};
#[derive(Parser)]
#[clap(version = config::VERSION, author = config::AUTHOR_EMAIL, about = config::ABOUT)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[async_std::main]
async fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::List(list) => list.exec().await,
        SubCommand::Create(create) => create.exec().await,
        // SubCommand::Cat(cat) => cat.exec().await,
    };
}
