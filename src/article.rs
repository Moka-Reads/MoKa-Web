use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Options, html};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    title: String,
    pub slug: String,
    icon: String,
    date: String,
    tags: String,
    content: String,
}

impl Article {
    pub fn new(markdown: &str) -> Self{
        // Parse the metadata block
        let meta_prefix = "---";
        let meta_delimiter = ":";

        let mut title = String::new();
        let mut slug = String::new();
        let mut content = String::new();

        let mut is_metadata = false;
        let mut date = String::new();
        let mut tags = Vec::new();
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
                            "icon" => icon = value.trim().to_string(),
                            "date" => date = value.trim().to_string(),
                            "tags" => tags = value.trim().split(',').map(|tag| tag.trim().to_string()).collect(),
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
            icon,
            date,
            tags: tags.join(","),
            content: html_output,
        }
    }
}
