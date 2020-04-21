use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Display, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(rename = "fermentation_status"))]
#[cfg_attr(feature = "db", sqlx(rename_all = "lowercase"))]
pub enum FermentationStatus {
    Primary,
    Secondary,
}

mod from_str {
    use super::FermentationStatus;
    use anyhow::Error;
    use std::str::FromStr;

    impl FromStr for FermentationStatus {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Primary" => Ok(FermentationStatus::Primary),
                "Secondary" => Ok(FermentationStatus::Secondary),
                _ => Err(Error::msg("Invalid fermentation")),
            }
        }
    }
}
