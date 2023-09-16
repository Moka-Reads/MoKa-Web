use dir::{Files, ResourceRoutes};
use handles::*;
use mokareads_core::awesome_lists::AwesomeList;
use mokareads_core::resources::article::articles_rss;
use rocket::fs::FileServer;
use rocket::{catchers, launch, routes};
use rocket_dyn_templates::Template;
use rocket::tokio;
use mokareads_core::resources::Cacher;

/// Provides the resources file walker and the cacher to load them all
pub mod dir;
/// Provides an abstraction for downloading files
mod downloader;
/// All of the different route handles for the website
pub mod handles;
pub mod page;
/// The roadmap type for the toml file
mod roadmap;

/// Initializes the global Cacher as well as the Resource Routes, and RSS
async fn init() -> Cacher {
    let files = Files::new().await.unwrap_or_default();
    let c = Cacher::new(files.articles(), files.cheatsheets(), files.guides());

    // Save the resource routes used for load balancing testing
    let res_routes = ResourceRoutes::new(&c);
    res_routes.save().await.unwrap();

    // Grabs the articles from the cached file
    let articles = c.articles();
    // Uses the array of articles to turn it into rss Channel
    let channel = articles_rss(articles);
    // Create a synchronous writer as the file we want to write to
    let writer = std::fs::File::create("moka_articles.rss").unwrap();
    // Format and write the RSS file
    channel.pretty_write_to(writer, b' ', 2).unwrap();
    c
}
/// Initializes the awesome lists with default 10 pages
async fn init_al() -> AwesomeList {
    let awesome_lists = AwesomeList::new(10).await.unwrap();
    awesome_lists
}

#[launch]
async fn rocket() -> _ {
    // Unwraps are necessary here because if the cacher and resource routes aren't
    // saved then we will have issues running the website, which is why they must exist
    // before the website begins.

    // Spawn each init task into asynchronous threads then run them in parallel using `join!`
    let init_task = tokio::spawn(init());
    let init_al_task = tokio::spawn(init_al());
    let joined = tokio::join!(init_task, init_al_task);

    let cacher = joined.0.unwrap();
    let awesome_lists = joined.1.unwrap();

    // Runs our web server with the given tera engine, web handles, and catchers
    rocket::build()
        .attach(Template::fairing()) // Attach the tera engine to the web server
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
                awesome_home,
                awesome_page,
                downloader_app,
                downloads_home,
                api_resources,
                api_cheatsheets,
                api_guides,
                api_articles,
                api_awesome,
                api_lang_map,
            ],
        ) // Mount the routes for the website
        .mount("/assets", FileServer::from("assets")) // Mount the assets folder for static files
        .register("/", catchers![not_found, internal_error]) // Register the catchers for 404 and 500 errors
        .manage(cacher) // Manage the cacher as a global state
        .manage(awesome_lists) // Manage the awesome lists as a global state
}
