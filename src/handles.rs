use rocket::get;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use crate::article::Article;
use crate::cheatsheet::Language::*;
use crate::cheatsheet::sort_the_cheats;
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

    let article = articles.iter().find(|x| x.slug == slug.trim());

    Template::render("article", context! {
        article: article.unwrap()
    })
}

#[get("/guides/<repo>")]
pub async fn guide_(repo: &str) -> Redirect{
    let files = Files::new().await.unwrap();
    let guides = files.guides();

    let guide = guides.iter().find(|x| x.repo_name == repo).unwrap();
    guide.redirect()
}

#[get("/cheatsheets")]
pub async fn cheatsheet_home() -> Template{
    let files = Files::new().await.unwrap();
    let cheatsheets = files.cheatsheets();

    let kotlin = sort_the_cheats(Kotlin, &cheatsheets);
    let rust = sort_the_cheats(Rust, &cheatsheets);
    let python = sort_the_cheats(Python, &cheatsheets);
    let c = sort_the_cheats(C, &cheatsheets);
    let cpp = sort_the_cheats(CPP, &cheatsheets);
    let zig = sort_the_cheats(Zig, &cheatsheets);
    let swift = sort_the_cheats(Swift, &cheatsheets);
    let go = sort_the_cheats(Go, &cheatsheets);
    let other = sort_the_cheats(Other, &cheatsheets);


    Template::render("cheatsheet_home", context! {
        rust: rust,
        kotlin: kotlin,
        python: python,
        c: c,
        cpp: cpp,
        zig: zig,
        swift: swift,
        go: go,
        other: other,
    })
}