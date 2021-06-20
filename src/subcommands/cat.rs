use super::Executable;
use async_trait::async_trait;
use clap::Clap;

pub const DESCRIPTION: &str =
    "Check out the contents of the gitignore template for given language or technology";

#[derive(Clap)]
#[clap(about = DESCRIPTION)]
pub struct Cat {
    language: String,
}

#[async_trait]
impl Executable for Cat {
    async fn exec(self) -> () {
        println!("Catting a {} gitignore file.", self.language);
    }
}
