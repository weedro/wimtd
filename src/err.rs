use std::fmt::Debug;

pub enum Error {
    Fetch,
    Database(sqlite::Error)
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch => write!(f, "Failed to fetch active window"),
            Self::Database(arg0) => f.debug_tuple("Database error").field(arg0).finish(),
        }
    }
}

impl From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Self {
        Error::Database(e)
    }
}