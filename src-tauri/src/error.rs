pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Surreal(surrealdb::Error),
    StoreFailToCreate(String),
    ErrGettingStore,
    ErrLimitOutOfBonds,
    XPropertyNotFound(String),
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
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
