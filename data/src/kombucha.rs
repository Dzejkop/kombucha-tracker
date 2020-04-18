use super::Entry;
use crate::KombuchaId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Kombucha {
    pub id: KombuchaId,
    pub name: String,
    pub added: DateTime<Utc>,
    pub entries: Vec<Entry>,
}

impl Kombucha {
    pub fn new_without_id(name: impl ToString, added: DateTime<Utc>, entries: Vec<Entry>) -> Self {
        Self {
            id: 0.into(),
            name: name.to_string(),
            added,
            entries,
        }
    }
}
