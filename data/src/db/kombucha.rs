use crate::id::KombuchaId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Kombucha {
    pub id: KombuchaId,
    pub name: String,
    pub added: DateTime<Utc>,
}
