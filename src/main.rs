use dir::{Files, ResourceRoutes};
use handles::*;
use mokareads_core::awesome_lists::AwesomeList;
use mokareads_core::resources::article::articles_rss;
use mokareads_core::resources::{Cacher, SearchMetadata, Searcher};
use rocket::tokio;
use rocket::{catchers, launch, routes, Build, Rocket};
use rocket_dyn_templates::Template;
use tokio::sync::Mutex;

/// Cached files for static assets
pub mod cached;
/// Provides the resources file walker and the cacher to load them all
pub mod dir;
/// Provides an abstraction for downloading files
mod downloader;
/// All of the different route handles for the website
#[allow(renamed_and_removed_lints)]
pub mod handles;
/// Pagination abstraction
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
/// Initializes the awesome list from generated `awesome.json`
async fn init_al() -> AwesomeList {
    let data = tokio::fs::read_to_string("awesome.json").await.unwrap();
    serde_json::from_str(&data).unwrap()
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // Unwraps are necessary here because if the cacher and resource routes aren't
    // saved then we will have issues running the website, which is why they must exist
    // before the website begins.

    // Spawn each init task into asynchronous threads then run them in parallel using `join!`
    let init_task = tokio::spawn(init());
    let init_al_task = tokio::spawn(init_al());
    let joined = tokio::join!(init_task, init_al_task);

    let cacher = joined.0.unwrap();
    let awesome_lists = joined.1.unwrap();
    let searcher = Searcher::new(&cacher);
    let search_meta: Mutex<Vec<SearchMetadata>> = Mutex::new(Vec::new());

    // Runs our web server with the given tera engine, web handles, and catchers
    rocket::build()
        .attach(Template::fairing()) // Attach the tera engine to the web server
        .mount(
            "/",
            routes![
                index,
                assets,
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
                research,
                curr,
                search,
                search_results
            ],
        ) // Mount the routes for the website
        .register("/", catchers![not_found, internal_error]) // Register the catchers for 404 and 500 errors
        .manage(cacher.articles()) // Manage the articles as a global state
        .manage(cacher.cheatsheets()) // Manage the cheatsheets as a global state
        .manage(cacher.guides()) // Manage the guides as a global state
        .manage(cacher) // Manage the cacher as a global state
        .manage(awesome_lists) // Manage the awesome lists as a global state
        .manage(searcher) // Manages the global state of Searcher
        .manage(search_meta) // Manages cache for search results with `SearchMetadata`
}
