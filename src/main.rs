use dir::ResourceRoutes;
use mokareads_core::resources::article::articles_rss;
use rocket::fs::FileServer;
use rocket::{catchers, launch, routes};
use rocket_dyn_templates::Template;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Provides the resources file walker and the cacher to load them all
pub mod dir;
/// Provides an abstraction to handle the Guides in MoKa Reads
pub mod guide;
/// All of the different route handles for the website
pub mod handles;
/// The roadmap type for the toml file
mod roadmap;

use crate::dir::Cacher;
use handles::*;
use lazy_static::lazy_static;

lazy_static! {
    /// Lazily evaluated cacher for MoKa Reads Resources
    /// Allows to lazily evaluate the resources without needing to keep reading `index.json`
    /// `RwLock` allows for multithreaded reading while writing will be fair
    /// `Arc` allows for multiple owners to exist concurrently
    pub static ref CACHER: Arc<RwLock<Cacher>> = Arc::new(RwLock::new(Cacher::default()));
}

/// Initializes the global Cacher as well as the Resource Routes, and RSS
async fn init() {
    let c = Cacher::new().await;

    // Save the resource routes used for load balancing testing
    let res_routes = ResourceRoutes::new(&c);
    res_routes.save().await.unwrap();

    // Grabs the articles from the cached file
    let articles = c.articles();
    // Uses the array of articles to turn it into rss Channel
    let channel = articles_rss(articles);
    // Create a synchronous writer as the file we want to write to
    let writer = std::fs::File::create("resources/moka_articles.rss").unwrap();
    // Format and write the RSS file
    channel.pretty_write_to(writer, b' ', 2).unwrap();

    let mut cacher = CACHER.write().await;
    *cacher = c;
}

#[launch]
async fn rocket() -> _ {
    // Unwraps are necessary here because if the cacher and resource routes aren't
    // saved then we will have issues running the website, which is why they must exist
    // before the website begins.

    init().await;

    // Runs our web server with the given tera engine, web handles, and catchers
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                mission,
                licenses,
                license,
                article_home,
                article_,
                cheatsheet_home,
                guides,
                guide_,
                cheatsheet_,
                rss,
                tools
            ],
        )
        .mount("/assets", FileServer::from("assets"))
        .register("/", catchers![not_found, internal_error])
}
