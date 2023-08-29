use crate::guide::Guide;
use mokareads_core::resources::{article::Article, cheatsheet::Cheatsheet, Parser};
use serde::{Deserialize, Serialize};
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
    let path = Path::new("./resources").join(folder);
    let mut entries = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let content = read_to_string(entry.path()).await?;
            entries.push(content);
        }
    }

    Ok(entries)
}

fn get_dir_names(folder: &str) -> Result<Vec<String>> {
    let mut entries = Vec::new();

    let path = Path::new("./resources").join(folder);

    for entry in WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .skip(1)
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
                entries.push(dir_name.to_string());
            }
        }
    }

    Ok(entries)
}

impl Files {
    pub async fn new() -> Result<Self> {
        let tasks = tokio::join!(get_files(ARTICLES), get_files(CHEATSHEETS));
        let articles = tasks.0?;
        let cheatsheets = tasks.1?;
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cacher {
    articles: Vec<Article>,
    cheatsheets: Vec<Cheatsheet>,
    guides: Vec<Guide>,
}

impl Cacher {
    pub async fn new() -> Self {
        let files = Files::new().await.unwrap();
        let articles = files.articles();
        let cheatsheets = files.cheatsheets();
        let guides = files.guides();
        Self {
            articles,
            cheatsheets,
            guides,
        }
    }

    pub async fn load() -> Self {
        let index_json = read_to_string("./resources/index.json").await.unwrap();
        let cacher: Cacher = serde_json::from_str(&index_json).unwrap();
        cacher
    }

    pub async fn save(&self) -> Result<()> {
        let index_json = serde_json::to_string_pretty(&self).unwrap();
        tokio::fs::write("./resources/index.json", index_json).await?;
        Ok(())
    }

    pub fn articles(&self) -> Vec<Article> {
        self.articles.clone()
    }
    pub fn guides(&self) -> Vec<Guide> {
        self.guides.clone()
    }
    pub fn cheatsheets(&self) -> Vec<Cheatsheet> {
        self.cheatsheets.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceRoutes {
    routes: Vec<String>,
}

impl ResourceRoutes {
    pub fn new(cacher: &Cacher) -> Self {
        let mut routes = vec![];
        // do articles first
        for a in &cacher.articles {
            routes.push(format!("/articles/{}", a.slug))
        }
        for c in &cacher.cheatsheets {
            routes.push(format!("/cheatsheets/{}", c.slug))
        }
        for g in &cacher.guides {
            routes.push(format!("/guides/{}", g.repo_name))
        }
        Self { routes }
    }
    pub async fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self).unwrap();
        tokio::fs::write("./resources/resources.json", data).await?;
        Ok(())
    }
}
