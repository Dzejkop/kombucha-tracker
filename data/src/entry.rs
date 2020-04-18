use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub added: DateTime<Utc>,
    pub content: String,
}

impl Entry {
    pub fn new(added: DateTime<Utc>, text: String) -> Self {
        Self {
            added,
            content: text,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new(Utc::now(), "".to_string())
    }
}
