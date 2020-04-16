use super::fermentation::Fermentation;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Kombucha {
    pub name: String,
    pub added: DateTime<Utc>,
    pub status: Fermentation,
}
