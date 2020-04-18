use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Kombucha {
    pub id: i32,
    pub name: String,
    pub added: DateTime<Utc>,
}
