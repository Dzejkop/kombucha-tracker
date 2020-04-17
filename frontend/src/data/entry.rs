use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    pub added: DateTime<Utc>,
    pub text: String,
}

impl Entry {
    pub fn new(added: DateTime<Utc>, text: String) -> Self {
        Self { added, text }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new(Utc::now(), "".to_string())
    }
}
