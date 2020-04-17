use super::{fermentation::Fermentation, Entry};
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Kombucha {
    pub name: String,
    pub added: DateTime<Utc>,
    pub status: Fermentation,
    pub entries: Vec<Entry>,
}

impl Kombucha {
    pub fn test_default() -> Self {
        Kombucha {
            name: "kombucha".to_string(),
            added: Utc::now(),
            status: Fermentation::Primary,
            entries: vec![Entry {
                added: Utc::now(),
                text: "Started making this kombucha".to_string(),
            }],
        }
    }

    pub fn default_new() -> Self {
        Kombucha {
            name: "New kombucha".to_string(),
            added: Utc::now(),
            status: Fermentation::Primary,
            entries: vec![],
        }
    }
}
