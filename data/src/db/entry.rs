use crate::{id::EntryId, KombuchaId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Entry {
    pub id: EntryId,
    pub kombucha_id: KombuchaId,
    pub content: String,
    pub added: DateTime<Utc>,
}
