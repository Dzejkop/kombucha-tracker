use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Entry {
    pub id: i32,
    pub kombucha_id: i32,
    pub content: String,
    pub added: DateTime<Utc>,
}
