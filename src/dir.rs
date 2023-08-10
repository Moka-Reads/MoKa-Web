use crate::guide::Guide;
use mokareads_core::resources::{article::Article, cheatsheet::Cheatsheet, Parser};
use std::io::Result;
use std::path::Path;
use tokio::fs::read_to_string;
use walkdir::WalkDir;

const ARTICLES: &str = "Moka-Articles";
const CHEATSHEETS: &str = "Moka-Cheatsheets";
const GUIDES: &str = "Moka-Guides";

#[derive(Debug, Clone)]
pub struct Files {
    articles: Vec<String>,
    cheatsheets: Vec<String>,
    guides: Vec<String>,
}
async fn get_files(folder: &str) -> Result<Vec<String>> {
    let mut entries = Vec::new();
    let path = Path::new("./resources").join(folder);
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.to_str().unwrap().contains("md"){
            let content = read_to_string(entry.path()).await?;
                entries.push(content)
        }
    }
    Ok(entries)
}

fn get_dir_names(folder: &str) -> Result<Vec<String>> {
    let mut entries = Vec::new();

    let path = Path::new("./resources").join(folder);
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let dir_name = dir_name.to_str().unwrap();
            entries.push(dir_name.to_string());
        }
    }

    Ok(entries)
}

impl Files {
    pub async fn new() -> Result<Self> {
        let articles = get_files(ARTICLES).await?;
        let cheatsheets = get_files(CHEATSHEETS).await?;
        let guides = get_dir_names(GUIDES)?;
        Ok(Self {
            articles,
            cheatsheets,
            guides,
        })
    }
    pub fn articles(&self) -> Vec<Article> {
        let mut articles = Vec::new();
        for a in &self.articles {
            articles.push(Article::parse(a))
        }
        articles
    }
    pub fn guides(&self) -> Vec<Guide> {
        let mut guides = Vec::new();
        for g in &self.guides {
            guides.push(Guide::new(g))
        }
        guides
    }
    pub fn cheatsheets(&self) -> Vec<Cheatsheet> {
        let mut cheatsheets = Vec::new();
        for c in &self.cheatsheets {
            cheatsheets.push(Cheatsheet::parse(c))
        }
        cheatsheets
    }
}
