use mokareads_core::awesome_lists::{AwesomeList, Repository};
use mokareads_core::resources::article::Article;
use mokareads_core::resources::cheatsheet::{get_lang_map, Cheatsheet, Language};
use mokareads_core::resources::guide::Guide;
use mokareads_core::resources::{Cacher, Searcher, SearchMetadata};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{catch, fs::NamedFile, uri, State, post, FromForm};
use rocket::{get, Request};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;
use rocket::form::Form;
use rocket::tokio::sync::Mutex;

use crate::downloader::{Downloader, GitHubTag, Platforms, Version};
use crate::page::{current_page, Page};
use crate::roadmap::Roadmap;

/// A type alias for the Cacher global state
type StateCache = State<Cacher>;
/// A type alias for the AwesomeList global state
type StateAwesome = State<AwesomeList>;
/// A type alias for the Article global state
type StateArticle = State<Vec<Article>>;
/// A type alias for the Cheatsheet global state
type StateCheatsheet = State<Vec<Cheatsheet>>;
/// A type alias for the Guide global state
type StateGuide = State<Vec<Guide>>;

type SMState = State<Mutex<Vec<SearchMetadata>>>;

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
    NamedFile::open("moka_articles.rss").await.ok()
}
/// Loads all of the articles for the user to choose or search for
#[get("/articles")]
pub async fn article_home(articles: &StateArticle) -> Template {
    Template::render(
        "articles_home",
        context! {
            articles: articles.inner()
        },
    )
}
/// Given an article it will load the appropriate page
/// Unwrap is allowed here since they should be accessing this from the article's homepage
/// If they aren't and it is invalid they will be redirected to index
#[get("/articles/<slug>")]
pub async fn article_(slug: &str, articles: &StateArticle) -> Template {
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
pub async fn guides(guides: &StateGuide) -> Template {
    Template::render("howtoguide", context! {guides: guides.inner()})
}

/// Given a guide's repo name it will redirect to the proper github pages site
/// Unwrap is allowed here since they should be accessing this from the guide's homepage
/// If they aren't and it is invalid they will be redirected to index
#[get("/guides/<repo>")]
pub async fn guide_(repo: &str, guides: &StateGuide) -> Redirect {
    let guide = guides.iter().find(|x| x.repo_name == repo).unwrap();
    guide.redirect()
}

/// Loads all of the cheatsheet and gives arrays for each of the focused languages
/// and any cheatsheet that isn't part of them will be under the `Other` section.
#[get("/cheatsheets")]
pub async fn cheatsheet_home(cheatsheets: &StateCheatsheet) -> Template {
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
#[get("/cheatsheets/<lang>/<slug>")]
pub async fn cheatsheet_(lang: &str, slug: &str, cheatsheets: &StateCheatsheet) -> Template {
    let cheatsheet = match cheatsheets
        .iter()
        .find(|x| x.slug == slug.trim() && x.lang() == lang.trim())
    {
        Some(c) => c.clone(),
        None => Cheatsheet::default(),
    };
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
/// The homepage or first page of the awesome-lists
#[get("/awesome")]
pub async fn awesome_home(awesome_list: &StateAwesome) -> Template {
    let (list, pages) = page(1, awesome_list).await;
    Template::render(
        "awesome",
        context! {
            awesome_lists: list,
            pages: pages
        },
    )
}

/// The different pages of awesome-lists
#[get("/awesome/<page_num>")]
pub async fn awesome_page(page_num: usize, awesome_list: &StateAwesome) -> Template {
    let (list, pages) = page(page_num, awesome_list).await;
    Template::render(
        "awesome",
        context! {
            awesome_lists: list,
            pages: pages
        },
    )
}

/// Gets the proper repository and page
async fn page(page_num: usize, awesome_list: &StateAwesome) -> (Vec<Repository>, Vec<Page>) {
    let list = awesome_list.get_page(page_num);
    let mut pages = Page::pages();
    current_page(&mut pages, page_num);
    (list, pages)
}

/// Redirects the user to the proper download link for the given platform and version
#[get("/download/<platform>/<version>")]
pub async fn downloader_app(platform: String, version: String) -> Redirect {
    let platform = Platforms::parse(&platform).unwrap();
    let version = Version::parse(&version).unwrap();

    let downloader = Downloader::new(platform, version);
    let link = downloader.download_link();
    Redirect::to(link)
}

/// Loads the downloader homepage with the different versions
#[get("/download")]
pub async fn downloads_home() -> Template {
    let releases = GitHubTag::fetch_tags().await.unwrap();
    let mut version_vec = releases
        .iter()
        .map(|x| Version::parse(x).unwrap())
        .collect::<Vec<Version>>();
    version_vec.sort();
    let latest = version_vec.pop().unwrap().to_string();
    Template::render(
        "downloader",
        context! {
            releases: releases,
            latest: latest
        },
    )
}

/// Allows a user to download the resources index
#[get("/api/resources")]
pub async fn api_resources(cacher: &StateCache) -> Json<Cacher> {
    Json::from(cacher.inner().clone())
}

/// Allows a user to download the cheatsheets index
#[get("/api/cheatsheets")]
pub async fn api_cheatsheets(cheatsheets: &StateCheatsheet) -> Json<Vec<Cheatsheet>> {
    Json::from(cheatsheets.inner().clone())
}

/// Allows a user to download the articles index
#[get("/api/articles")]
pub async fn api_articles(articles: &StateArticle) -> Json<Vec<Article>> {
    Json::from(articles.inner().clone())
}

/// Allows a user to download the guides index
#[get("/api/guides")]
pub async fn api_guides(guides: &StateGuide) -> Json<Vec<Guide>> {
    Json::from(guides.inner().clone())
}

/// Allows a user to download the awesome-lists index
#[get("/api/awesome")]
pub async fn api_awesome(awesome_list: &StateAwesome) -> Json<AwesomeList> {
    Json::from(awesome_list.inner().clone())
}

/// Allows a user to download the language map for the cheatsheets
#[get("/api/lang_map")]
pub async fn api_lang_map(cheatsheets: &StateCheatsheet) -> Json<HashMap<Language, Vec<Cheatsheet>>> {
    Json::from(get_lang_map(cheatsheets))
}

/// Allows a user to view the MoKa Resarch iniative 
#[get("/research")]
pub async fn research() -> Template{
    Template::render("research", context!{})
}

/// View for curriculum plan 
#[get("/research/curr/<code>")]
pub async fn curr(code: &str) -> Template{
    let view = format!("courses/{code}");
    Template::render(view, context!{})
}

/// Search bar input form
#[derive(FromForm)]
pub struct InputForm{
    search: String
}

/// Searches for resources either by their language, title, and resource type
#[post("/", data="<form>")]
pub async fn search(form: Form<InputForm>, metadata_state: &SMState, searcher_state: &State<Searcher>) -> Redirect{
    let input = form.search.to_string();
    let result = searcher_state.search(input);
    let mut metadata = metadata_state.inner().lock().await;
    *metadata = result;
    Redirect::to(uri!(search_results))
}

/// Redirect page to see the search results 
#[get("/search")]
pub async fn search_results(metadata_state: &SMState) -> Template{
    let metadata = metadata_state.lock().await;
    Template::render("search_results", context! {
        results: metadata.clone()
    })
}

