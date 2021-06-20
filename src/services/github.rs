use crate::config::{FILE_NAME_SELECTOR, RAW_URL, SOURCE_URL};
use scraper::{Html, Selector};

pub struct GithubService {}

impl GithubService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_gitignore(&self, language: String) -> String {
        let mut res = surf::get(RAW_URL.replace("{}", &language)).await.unwrap();
        let content = &res.body_string().await.unwrap();

        content.into()
    }

    pub async fn list_languages(&self) -> Vec<String> {
        let mut res = surf::get(SOURCE_URL).await.unwrap();

        let document = Html::parse_document(&res.body_string().await.unwrap());
        let selector = Selector::parse(FILE_NAME_SELECTOR).unwrap();

        let mut file_names = Vec::<String>::new();

        for element in document.select(&selector) {
            let file_name = element.text().collect::<Vec<_>>()[0];

            if file_name.ends_with(".gitignore") {
                file_names.push(file_name[..file_name.len() - 10].into());
            }
        }

        file_names
    }
}
