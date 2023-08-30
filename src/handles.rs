use crate::roadmap::Roadmap;
use mokareads_core::resources::cheatsheet::{get_lang_map, Language};
use rocket::response::Redirect;
use rocket::{catch, fs::NamedFile, uri};
use rocket::{get, Request};
use rocket_dyn_templates::{context, Template};
use crate::CACHER;

/// The homepage of the website to present the idea of MoKa Reads and Opensource Education
#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}
/// Provides the mission of the MoKa Reads platform
/// We also read the roadmap toml file to present the current roadmap
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
/// Provides information for the different licenses for the MoKa Reads Platform
#[get("/licenses")]
pub fn licenses() -> Template {
    Template::render("license_home", context! {})
}
/// Opens the page for the given license from the license homepage
/// The logic is handled with an if/else statement in the template
#[get("/licenses/<license>")]
pub fn license(license: &str) -> Template {
    Template::render("license", context! {license: license})
}
/// Opens the rss file which contains all of the different articles from MoKa Reads
#[get("/rss")]
pub async fn rss() -> Option<NamedFile> {
    NamedFile::open("resources/moka_articles.rss").await.ok()
}
/// Loads all of the articles for the user to choose or search for
#[get("/articles")]
pub async fn article_home() -> Template {
    let cacher = CACHER.read().await;
    let articles = cacher.articles();
    Template::render(
        "articles_home",
        context! {
            articles: articles
        },
    )
}
/// Given an article it will load the appropriate page
/// Unwrap is allowed here since they should be accessing this from the article's homepage
/// If they aren't and it is invalid they will be redirected to index
#[get("/articles/<slug>")]
pub async fn article_(slug: &str) -> Template {
    let cacher = CACHER.read().await;
    let articles = cacher.articles();

    let article = articles.iter().find(|x| x.slug == slug.trim());

    Template::render(
        "article",
        context! {
            article: article.unwrap()
        },
    )
}
/// Loads all of the guides which the user can be redirected to
#[get("/guides")]
pub async fn guides() -> Template {
    let cacher = CACHER.read().await;
    let guides = cacher.guides();
    Template::render("howtoguide", context! {guides: guides})
}

/// Given a guide's repo name it will redirect to the proper github pages site
/// Unwrap is allowed here since they should be accessing this from the guide's homepage
/// If they aren't and it is invalid they will be redirected to index
#[get("/guides/<repo>")]
pub async fn guide_(repo: &str) -> Redirect {
    let cacher = CACHER.read().await;
    let guides = cacher.guides();

    let guide = guides.iter().find(|x| x.repo_name == repo).unwrap();
    guide.redirect()
}

/// Loads all of the cheatsheet and gives arrays for each of the focused languages
/// and any cheatsheet that isn't part of them will be under the `Other` section.
#[get("/cheatsheets")]
pub async fn cheatsheet_home() -> Template {
    let cacher = CACHER.read().await;
    let cheatsheets = cacher.cheatsheets();
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

/// Loads the given cheatsheet from the homepage
/// Unwrap is allowed here since they should be accessing this via the cheatsheet homepage
/// If they aren't and the cheatsheet is invalid they will be redirected to index
#[get("/cheatsheets/<slug>")]
pub async fn cheatsheet_(slug: &str) -> Template {
    let cacher = CACHER.read().await;
    let cheatsheets = cacher.cheatsheets();
    let cheatsheet = cheatsheets.iter().find(|x| x.slug == slug.trim()).unwrap();
    Template::render(
        "cheatsheet",
        context! {
            cheatsheet: cheatsheet
        },
    )
}

/// Redirects any page not found errors to homepage
/// This will help prevent any strange behaviour trying to happen with requests
#[catch(404)]
pub fn not_found(_req: &Request) -> Redirect {
    Redirect::to(uri!(index))
}

/// Redirects any internal errors to homepage
/// Subject to change in the future if post requests need to be made
#[catch(500)]
pub fn internal_error(_req: &Request) -> Redirect {
    Redirect::to(uri!(index))
}

// TODO: Implement tools pages for MoKaReads Template and Core!
#[get("/tools/<tool>")]
pub fn tools(tool: &str) -> String {
    match tool {
        _ => "Sorry but this is still in development :(".to_string(),
    }
}
