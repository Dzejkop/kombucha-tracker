use crate::{id::FermentationId, FermentationStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Fermentation {
    pub id: FermentationId,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub est_end_date: Option<DateTime<Utc>>,
    pub status: FermentationStatus,
}
