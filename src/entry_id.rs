use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct EntryId(String);

#[derive(Debug, Eq, Error, PartialEq)]
#[error("entry id parse error")]
pub struct EntryIdParseError;

impl std::fmt::Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EntryId {
    type Err = EntryIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            "2500000000".parse::<EntryId>(),
            Ok(EntryId("2500000000".to_string()))
        );
        assert_eq!(
            EntryId::from_str("2500000000").map(|id| id.to_string()),
            Ok("2500000000".to_string())
        );
    }
}
