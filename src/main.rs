use rocket::{launch, routes};
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template};

pub mod handles;
mod roadmap;
pub mod dir;
pub mod article;

use handles::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, mission, licenses, license_handle, article_home, article_])
        .mount("/assets", FileServer::from("assets"))
}
