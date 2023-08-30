use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guide {
    pub repo_name: String,
    pub unslug: String,
    pub addy: String,
}

impl Guide {
    pub fn new(repo_name: &str) -> Self {
        Self {
            repo_name: repo_name.to_string(),
            unslug: repo_name.replace('_', " "),
            addy: Guide::redirect_address(repo_name),
        }
    }
    pub fn redirect_address(repo_name: &str) -> String {
        format!("https://moka-reads.github.io/{}/", repo_name)
    }
    pub fn redirect(&self) -> Redirect {
        let address = self.addy.to_string();
        Redirect::to(address)
    }
}
