use std::fmt::Debug;

pub enum Error {
    Fetch,
    Sync(reqwest::blocking::Response),
    Connection(reqwest::Error),
    Serialization(serde_json::Error),
    Database(sqlite::Error)
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch => write!(f, "Failed to fetch active window"),
            Self::Sync(r) => f.debug_tuple("Failed to sync records").field(r).finish(),
            Self::Connection(e) => f.debug_tuple("Failed to sync records: connection error").field(e).finish(),
            Self::Serialization(e) => f.debug_tuple("Failed to sync records: json serialization error").field(e).finish(),
            Self::Database(e) => f.debug_tuple("Database error").field(e).finish(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Connection(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialization(e)
    }
}

impl From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Self {
        Error::Database(e)
    }
}
