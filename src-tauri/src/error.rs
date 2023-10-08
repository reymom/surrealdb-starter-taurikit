pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Surreal(surrealdb::Error),
    StoreFailToCreate(String),
    StoreFailToPatch {
        method: String,
        tb: String,
        tid: String,
    },
    ErrGettingStore,
    ErrLimitOutOfBonds,
    XValueNotFound(String),
    XValueNotOfType(&'static str),
}

impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        Error::Surreal(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        match self {
            Error::StoreFailToPatch { method, tb, tid } => {
                write!(fmt, "error in {method} for {tb}:{tid}")
            }
            _ => write!(fmt, "{self:?}"),
        }
    }
}

impl std::error::Error for Error {}
