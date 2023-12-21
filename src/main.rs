use handles::*;
use rocket::{catchers, launch, routes, Build, Rocket};
use rocket_dyn_templates::Template;

/// Cached files for static assets
pub mod cached;
/// Provides the resources file walker and the cacher to load them all
//pub mod dir;
/// Provides an abstraction for downloading files
//mod downloader;
/// All of the different route handles for the website
#[allow(renamed_and_removed_lints)]
pub mod handles;
/// Pagination abstraction
//pub mod page;
/// The roadmap type for the toml file
mod roadmap;

#[launch]
async fn rocket() -> Rocket<Build> {
    // Runs our web server with the given tera engine, web handles, and catchers
    rocket::build()
        .attach(Template::fairing()) // Attach the tera engine to the web server
        .mount(
            "/",
            routes![
                index,
                assets,
                mission,
                policies, 
                licenses, 
            ],
        ) // Mount the routes for the website
        .register("/", catchers![not_found, internal_error]) // Register the catchers for 404 and 500 errors
}
