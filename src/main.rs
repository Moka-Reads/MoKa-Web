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

    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                mission,
                licenses,
                article_home,
                article_,
                cheatsheet_home,
                guides,
                guide_,
                cheatsheet_
            ],
        )
        .mount("/assets", FileServer::from("assets"))
}
