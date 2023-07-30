use rocket::{get, launch, routes};
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};

pub mod db;
pub mod handles;
mod roadmap;

use handles::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, mission, licenses, license_handle])
        .mount("/assets", FileServer::from("assets"))
}
