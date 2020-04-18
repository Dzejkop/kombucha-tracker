use super::{fermentation::Fermentation, Entry};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
                content: "Started making this kombucha".to_string(),
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
