use mongodb::bson::doc;
use rocket::get;
use rocket_dyn_templates::{context, Template};
use crate::roadmap::Roadmap;


#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/mission")]
pub async fn mission() -> Template{
    let mut rmap = Roadmap::new().await;
    rmap.sort();
    Template::render("mission", context! {
        roadmap: rmap.map_items
    })
}

#[get("/licenses")]
pub fn licenses() -> Template{
    Template::render("license_home", context! {})
}

#[get("/licenses/<lic>")]
pub fn license_handle(lic: &str) -> Template{
    Template::render("license", context! {license: lic})
}