use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Guide {
    pub repo_name: String,
}

impl Guide {
    pub fn new(repo_name: &str) -> Self {
        Self {
            repo_name: repo_name.to_string(),
        }
    }
    pub fn redirect_address(&self) -> String {
        format!("https://moka-reads.github.io/{}/", self.repo_name)
    }
    pub fn redirect(&self) -> Redirect {
        let address = self.redirect_address();
        Redirect::to(address)
    }
}
