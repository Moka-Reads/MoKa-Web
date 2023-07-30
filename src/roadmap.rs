use std::cmp::Ordering;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::error::Result;
use tokio::fs::read_to_string;
use crate::db::connect;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct RoadmapItem{
    pub year: u32,
    pub period: Period,
    pub description: String
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Period{
    Q1,
    Q2,
    Q3,
    Q4
}

impl Ord for Period {
    fn cmp(&self, other: &Self) -> Ordering {
        // Define the custom ordering for the Period enum
        // The ordering will be Q1 < Q2 < Q3 < Q4
        match (self, other) {
            (Period::Q1, Period::Q1) => Ordering::Equal,
            (Period::Q1, _) => Ordering::Less,
            (Period::Q2, Period::Q2) => Ordering::Equal,
            (Period::Q2, Period::Q1) => Ordering::Greater,
            (Period::Q2, _) => Ordering::Less,
            (Period::Q3, Period::Q3) => Ordering::Equal,
            (Period::Q3, (Period::Q1 | Period::Q2)) => Ordering::Greater,
            (Period::Q3, _) => Ordering::Less,
            (Period::Q4, Period::Q4) => Ordering::Equal,
            (Period::Q4, _) => Ordering::Greater,
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
        // First, compare the periods
        let period_ordering = self.period.cmp(&other.period);
        if period_ordering != Ordering::Equal {
            return period_ordering;
        }

        // If the periods are the same, compare the years
        self.year.cmp(&other.year)
    }
}

impl PartialOrd for RoadmapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Roadmap{
    pub map_items: Vec<RoadmapItem>
}

impl Roadmap {
    pub async fn new() -> Self{
        let str = read_to_string("roadmap.toml").await.unwrap();
        toml::from_str(&str).unwrap()
    }
    pub fn sort(&mut self) {
        self.map_items.sort_by(|a, b| a.cmp(b))
    }
}