use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub content: String,
    pub categories: Vec<String>,
}
