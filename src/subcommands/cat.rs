use super::Executable;
use clap::Clap;

pub const DESCRIPTION: &str = "(show) Check out the contents of the template for given language";

#[derive(Clap)]
#[clap(about = DESCRIPTION, alias = "show")]
pub struct Cat {
    language: String,
}

impl Executable for Cat {
    fn exec(self) -> () {
        println!("Catting a {} gitignore file.", self.language);
    }
}
