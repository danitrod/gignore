use super::Executable;
use clap::Clap;

pub const DESCRIPTION: &str = "(new) Create a template for a given language";

#[derive(Clap)]
#[clap(about = DESCRIPTION, alias = "new")]
pub struct Create {
    language: String,
}

impl Executable for Create {
    fn exec(self) -> () {
        println!("Creating a {} gitignore file.", self.language);
    }
}
