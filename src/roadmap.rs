use crate::db::connect;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tokio::fs::read_to_string;
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct RoadmapItem {
    pub year: u32,
    pub period: Period,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Period {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Ord for Period {
    fn cmp(&self, other: &Self) -> Ordering {
        use Period::*;
        match (self, other) {
            (Q1, Q1) => Ordering::Equal,
            (Q1, _) => Ordering::Less,
            (Q2, Q2) => Ordering::Equal,
            (Q2, Q1) => Ordering::Greater,
            (Q2, _) => Ordering::Less,
            (Q3, Q3) => Ordering::Equal,
            (Q3, Q1) | (Q3, Q2) => Ordering::Greater,
            (Q3, _) => Ordering::Less,
            (Q4, Q4) => Ordering::Equal,
            (Q4, _) => Ordering::Greater,
        }
    }
}


impl PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoadmapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare the years
        let year_ordering = self.year.cmp(&other.year);
        if year_ordering != Ordering::Equal {
            return year_ordering;
        }

        // If the years are the same, compare the periods
        self.period.cmp(&other.period)
    }
}

impl PartialOrd for RoadmapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Roadmap {
    pub map_items: Vec<RoadmapItem>,
}

impl Roadmap {
    pub async fn new() -> Self {
        let str = read_to_string("roadmap.toml").await.unwrap();
        toml::from_str(&str).unwrap()
    }
    pub fn sort(&mut self) {
        self.map_items.sort_by(|a, b| a.cmp(b))
    }
}
