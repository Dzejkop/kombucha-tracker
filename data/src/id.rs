use derive_more::{Display, From, FromStr, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Clone, Copy, Deserialize, Serialize, From, Into, FromStr)]
#[cfg_attr(feature = "db", derive(sqlx::Type), sqlx(transparent))]
pub struct KombuchaId(i32);

#[derive(Debug, Display, PartialEq, Clone, Copy, Deserialize, Serialize, From, Into, FromStr)]
#[cfg_attr(feature = "db", derive(sqlx::Type), sqlx(transparent))]
pub struct EntryId(i32);

#[derive(Debug, Display, PartialEq, Clone, Copy, Deserialize, Serialize, From, Into, FromStr)]
#[cfg_attr(feature = "db", derive(sqlx::Type), sqlx(transparent))]
pub struct FermentationId(i32);