use crate::id::EntryId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub added: DateTime<Utc>,
    pub content: String,
}

impl Entry {
    pub fn new(id: EntryId, added: DateTime<Utc>, text: String) -> Self {
        Self {
            id,
            added,
            content: text,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new(EntryId::from(0), Utc::now(), "".to_string())
    }
}

#[cfg(feature = "db")]
mod db {
    use super::Entry;
    use crate::db::Entry as DbEntry;

    impl From<DbEntry> for Entry {
        fn from(
            DbEntry {
                id, added, content, ..
            }: DbEntry,
        ) -> Self {
            Self { id, added, content }
        }
    }
}
