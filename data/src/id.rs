use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize, From, Into)]
#[cfg_attr(feature = "db", derive(sqlx::Type), sqlx(transparent))]
pub struct KombuchaId(i32);

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize, From, Into)]
#[cfg_attr(feature = "db", derive(sqlx::Type), sqlx(transparent))]
pub struct EntryId(i32);
