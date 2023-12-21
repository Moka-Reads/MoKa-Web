use rocket::response::Redirect;
use rocket::{catch, fs::NamedFile,uri};
use rocket::{get, Request};
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};
use crate::cached::CachedNameFile;

use crate::roadmap::Roadmap;

const VERSION: &str = env!("CARGO_PKG_VERSION");


/// The homepage of the website to present the idea of MoKa Reads and Opensource Education
#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        version: VERSION.to_string(), 
    })
}

/// Opens a asset file as cached with max age of 3600
#[get("/assets/<file..>")]
pub async fn assets(file: PathBuf) -> Option<CachedNameFile>{
    NamedFile::open(Path::new("assets/").join(file))
        .await
        .ok()
        .map(|file| CachedNameFile::max_age(file, 31536000        ))
}

/// Provides the mission of the MoKa Reads platform
/// We also read the roadmap toml file to present the current roadmap
#[get("/mission")]
pub async fn mission() -> Template {
    let mut rmap = Roadmap::new().await;
    rmap.sort();
    Template::render(
        "mission",
        context! {
            roadmap: rmap.map_items,
            version: VERSION.to_string()
        },
    )
}

#[get("/policies")]
pub async fn policies() -> Template{
    let opencode_url = "https://raw.githubusercontent.com/Moka-Reads/OpenCode/main/README.md";
    let req = reqwest::get(opencode_url).await.unwrap();
    let text = req.text().await.unwrap();
    let html = markdown::to_html(&text);


    Template::render("policies", context!{
        opencode: html, 
        version: VERSION.to_string(), 
    })
}

#[get("/licenses/<lic>")]
pub async fn licenses(lic: &str) -> Template{
    Template::render("license", context!{
        license: lic, 
        version: VERSION.to_string(), 
    })
}


/// Redirects any page not found errors to homepage
/// This will help prevent any strange behaviour trying to happen with requests
#[catch(404)]
pub fn not_found(_req: &Request) -> Redirect {
    Redirect::to(uri!(index))
}

/// Redirects any internal errors to homepage
/// Subject to change in the future if post requests need to be made
#[catch(500)]
pub fn internal_error(_req: &Request) -> Redirect {
    Redirect::to(uri!(index))
}

