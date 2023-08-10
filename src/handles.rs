use crate::dir::Files;
use crate::roadmap::Roadmap;
use mokareads_core::resources::cheatsheet::{get_lang_map, Language};
use rocket::get;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/mission")]
pub async fn mission() -> Template {
    let mut rmap = Roadmap::new().await;
    rmap.sort();
    Template::render(
        "mission",
        context! {
            roadmap: rmap.map_items
        },
    )
}

#[get("/licenses")]
pub fn licenses() -> Template {
    Template::render("license_home", context! {})
}

#[get("/licenses/<lic>")]
pub fn license_handle(lic: &str) -> Template {
    Template::render("license", context! {license: lic})
}

#[get("/articles")]
pub async fn article_home() -> Template {
    let files = Files::new().await.unwrap();
    let articles = files.articles();
    Template::render(
        "articles_home",
        context! {
            articles: articles
        },
    )
}

#[get("/articles/<slug>")]
pub async fn article_(slug: &str) -> Template {
    let files = Files::new().await.unwrap();
    let articles = files.articles();

    let article = articles.iter().find(|x| x.slug == slug.trim());

    Template::render(
        "article",
        context! {
            article: article.unwrap()
        },
    )
}

#[get("/guides")]
pub async fn guides() -> Template {
    let files = Files::new().await.unwrap();
    let _guides = files.guides();
    Template::render("howtoguide", context! {})
}

#[get("/guides/<repo>")]
pub async fn guide_(repo: &str) -> Redirect {
    let files = Files::new().await.unwrap();
    let guides = files.guides();

    let guide = guides.iter().find(|x| x.repo_name == repo).unwrap();
    guide.redirect()
}

#[get("/cheatsheets")]
pub async fn cheatsheet_home() -> Template {
    let files = Files::new().await.unwrap();
    let cheatsheets = files.cheatsheets();
    let lang_map = get_lang_map(&cheatsheets);

    let kotlin = lang_map
        .get(&Language::Kotlin)
        .unwrap_or(&Vec::new())
        .clone();
    let rust = lang_map.get(&Language::Rust).unwrap_or(&Vec::new()).clone();
    let python = lang_map
        .get(&Language::Python)
        .unwrap_or(&Vec::new())
        .clone();
    let c = lang_map.get(&Language::C).unwrap_or(&Vec::new()).clone();
    let cpp = lang_map.get(&Language::CPP).unwrap_or(&Vec::new()).clone();
    let zig = lang_map.get(&Language::Zig).unwrap_or(&Vec::new()).clone();
    let swift = lang_map
        .get(&Language::Swift)
        .unwrap_or(&Vec::new())
        .clone();
    let go = lang_map.get(&Language::Go).unwrap_or(&Vec::new()).clone();
    let other = lang_map
        .get(&Language::Other)
        .unwrap_or(&Vec::new())
        .clone();

    Template::render(
        "cheatsheet_home",
        context! {
            rust: rust,
            kotlin: kotlin,
            python: python,
            c: c,
            cpp: cpp,
            zig: zig,
            swift: swift,
            go: go,
            other: other,
        },
    )
}

#[get("/cheatsheets/<slug>")]
pub async fn cheatsheet_(slug: &str) -> Template {
    let files = Files::new().await.unwrap();
    let cheatsheets = files.cheatsheets();
    let cheatsheet = cheatsheets.iter().find(|x| x.slug == slug.trim()).unwrap();
    Template::render(
        "cheatsheet",
        context! {
            cheatsheet: cheatsheet
        },
    )
}
