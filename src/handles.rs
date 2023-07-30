use crate::db::connect;
use crate::resources::articles::Article;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use rocket::get;
use rocket_dyn_templates::{context, Template};
use crate::roadmap::Roadmap;

#[get("/articles/<id>")]
pub async fn article_handle(id: String) -> Template {
    // do something with db to confirm id
    let id = ObjectId::parse_str(id).unwrap();
    let collection: Collection<Article> = connect("articles").await.unwrap();
    let article = collection
        .find_one(doc! {"id": id}, None)
        .await
        .unwrap()
        .unwrap();
    Template::render("articles", context! {})
}

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