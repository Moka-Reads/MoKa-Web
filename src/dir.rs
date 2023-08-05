use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::io::Result;
use tokio::fs::read_to_string;
use crate::article::Article;

const ARTICLES: &str = "Moka-Articles";
const CHEATSHEETS: &str = "Moka-Cheatsheets";
const GUIDES: &str = "Moka-Guides";

#[derive(Debug, Clone)]
pub struct Files{
    articles: Vec<String>,
    cheatsheets: Vec<String>,
    guides: Vec<String>
}
async fn get_files(folder: &str) -> Result<Vec<String>>{
    let mut entries = Vec::new();
    let path = Path::new("resources").join(folder);
    for entry in WalkDir::new(path){
        let entry = entry?;
        let path = entry.path();
        if path.is_file(){
            if path.to_str().unwrap().contains("md"){
                let content = read_to_string(entry.path()).await?;
                entries.push(content)
            }

        }


    }
    Ok(entries)
}
impl Files {
    pub async fn new() -> Result<Self>{
        let articles = get_files(ARTICLES).await?;
        let cheatsheets = get_files(CHEATSHEETS).await?;
        let guides = get_files(GUIDES).await?;
        Ok(Self{articles, cheatsheets, guides})
    }
    pub fn articles(&self) -> Vec<Article>{
        let mut articles = Vec::new();
        for a in &self.articles{
            articles.push(Article::new(a))
        }
        articles
    }
}