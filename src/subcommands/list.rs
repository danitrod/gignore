use super::Executable;
use clap::Clap;

pub const DESCRIPTION: &str = "(ls) List the available template languages";

#[derive(Clap)]
#[clap(about = DESCRIPTION, alias = "ls")]
pub struct List {}

impl Executable for List {
    fn exec(self) -> () {
        println!("Listing possible files");
    }
}
