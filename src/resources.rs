pub mod articles;
pub mod cheatsheet;
pub mod howtoguide;

use crate::handles::article_handle;
use mongodb::bson::oid::ObjectId;
use rocket::response::Redirect;
use rocket::uri;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub title: String,
    pub description: String,
    pub published: String,
    id: ObjectId,
    resource: ResourceType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResourceType {
    Article(articles::Article),
    HowToGuide(howtoguide::HowToGuide),
    CheatSheet(cheatsheet::CheatSheet),
}
