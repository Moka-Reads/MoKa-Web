use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HowToGuide {
    pub repo: String,
}

impl HowToGuide {
    pub fn url(&self) -> String {
        format!("https://moka-reads.github.io/{}", &self.repo)
    }
}
