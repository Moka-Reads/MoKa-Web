use dir::ResourceRoutes;
use mokareads_core::resources::article::articles_rss;
use rocket::fs::FileServer;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

pub mod dir;
pub mod guide;
pub mod handles;
mod roadmap;

use handles::*;

#[launch]
async fn rocket() -> _ {
    let cacher = dir::Cacher::new().await;
    cacher.save().await.unwrap();

    let res_routes = ResourceRoutes::new(&cacher);
    res_routes.save().await.unwrap();

    let articles = cacher.articles();
    let channel = articles_rss(articles);
    let writer = std::fs::File::create("resources/moka_articles.rss").unwrap();
    channel.pretty_write_to(writer, b' ', 2).unwrap();

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
                rss
            ],
        )
        .mount("/assets", FileServer::from("assets"))
}
