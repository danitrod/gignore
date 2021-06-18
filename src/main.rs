use clap::{AppSettings, Clap};

mod config;
mod subcommands;

use subcommands::{Executable, SubCommand};
#[derive(Clap)]
#[clap(version = config::VERSION, author = config::AUTHOR_EMAIL)]
#[clap(setting = AppSettings::ColoredHelp, about = config::ABOUT)]

struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::List(list) => list.exec(),
        SubCommand::Create(create) => create.exec(),
        SubCommand::Cat(cat) => cat.exec(),
    }
}
