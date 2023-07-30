use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheatSheet {
    pub content: String,
    pub lang: String,
}
