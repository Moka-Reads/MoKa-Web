use rocket::get;
use rocket_dyn_templates::{context, Template};
use crate::article::Article;
use crate::dir::Files;
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

#[get("/articles")]
pub async fn article_home() -> Template{
    let files = Files::new().await.unwrap();
    let articles = files.articles();
    Template::render("articles_home", context! {
        articles: articles
    })
}

#[get("/articles/<slug>")]
pub async fn article_(slug: &str) -> Template{
    let files = Files::new().await.unwrap();
    let articles = files.articles();
    let mut article = None;

    for a in articles{
        if &a.slug == slug.trim(){
            article = Some(a)
        }
    }

    Template::render("article", context! {
        article: article.unwrap()
    })
}