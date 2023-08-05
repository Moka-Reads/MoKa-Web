use std::collections::HashMap;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheatsheet{
    title: String,
    pub slug: String,
    lang: String,
    icon: String,
    content: String,
}

impl Cheatsheet{
    pub fn new(markdown: &str) -> Self{
        // Parse the metadata block
        let meta_prefix = "---";
        let meta_delimiter = ":";

        let mut title = String::new();
        let mut slug = String::new();

        let mut content = String::new();
        let mut lang = String::new();

        let mut is_metadata = false;

        let mut icon = String::new();

        for line in markdown.lines() {
            if line.trim() == meta_prefix {
                is_metadata = !is_metadata;
            } else if is_metadata {
                // Process metadata
                let mut parts = line.trim().splitn(2, meta_delimiter);
                if let Some(key) = parts.next() {
                    if let Some(value) = parts.next() {
                        match key.trim() {
                            "title" => {
                                title = value.trim().to_string();
                                slug = value.trim().replace(" ", "_");
                            },
                            "lang" => lang = value.trim().to_lowercase(),
                            "icon" => icon = value.trim().to_string(),
                            _ => (),
                        }
                    }
                }
            } else {
                // Process content
                content.push_str(line);
                content.push('\n');
            }
        }

        let parser = Parser::new_ext(&content, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Self {
            title,
            slug,
            lang,
            icon,
            content: html_output,
        }
    }
}


#[derive(Debug,Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum Language{
    Kotlin,
    Rust,
    C,
    CPP,
    Zig,
    Python,
    Swift,
    Go,
    Other,
}

impl Language{
    pub fn from_str(s: &str) -> Language{
        match s{
            "kotlin" => Language::Kotlin,
            "rust" => Language::Rust,
            "c" => Language::C,
            "c++" => Language::CPP,
            "zig" => Language::Zig,
            "python" => Language::Python,
            "swift" => Language::Swift,
            "go" => Language::Go,
            _ => Language::Other
        }
    }
}

pub fn sort_the_cheats(lang: Language, cheatsheets: &Vec<Cheatsheet>) -> Vec<&Cheatsheet>{
    cheatsheets.iter().filter(|x|{
        Language::from_str(&x.lang) == lang
    }).collect()
}