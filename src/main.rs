use dir::ResourceRoutes;
use mokareads_core::resources::article::articles_rss;
use rocket::fs::FileServer;
use rocket::{catchers, launch, routes};
use rocket_dyn_templates::Template;

///! Provides the resources file walker and the cacher to load them all
pub mod dir;
///! Provides an abstraction to handle the Guides in MoKa Reads
pub mod guide;
///! All of the different route handles for the website
pub mod handles;
///! The roadmap type for the toml file
mod roadmap;

use handles::*;

#[launch]
async fn rocket() -> _ {
    // Unwraps are necessary here because if the cacher and resource routes aren't
    // saved then we will have issues running the website, which is why they must exist
    // before the website begins.

    // Creates a new cacher file that will allow us to quickly load the different resources
    let cacher = dir::Cacher::new().await;
    // save the file to `resources/index.json`
    cacher.save().await.unwrap();

    // Save the resource routes used for load balancing testing
    let res_routes = ResourceRoutes::new(&cacher);
    res_routes.save().await.unwrap();

    // Grabs the articles from the cached file
    let articles = cacher.articles();
    // Uses the array of articles to turn it into rss Channel
    let channel = articles_rss(articles);
    // Create a synchronous writer as the file we want to write to
    let writer = std::fs::File::create("resources/moka_articles.rss").unwrap();
    // Format and write the RSS file
    channel.pretty_write_to(writer, b' ', 2).unwrap();

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
