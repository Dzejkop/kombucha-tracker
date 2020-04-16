use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Display)]
pub enum Fermentation {
    Primary,
    Secondary,
}

mod from_str {
    use super::Fermentation;
    use anyhow::Error;
    use std::str::FromStr;

    impl FromStr for Fermentation {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Primary" => Ok(Fermentation::Primary),
                "Secondary" => Ok(Fermentation::Secondary),
                _ => Err(Error::msg("Invalid fermentation")),
            }
        }
    }
}
