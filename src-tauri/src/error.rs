pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Surreal(surrealdb::err::Error),
    StoreFailToCreate(String),
    ErrGettingStore,
    XPropertyNotFound(String),
    XValueNotOfType(&'static str),
}

impl From<surrealdb::err::Error> for Error {
    fn from(val: surrealdb::err::Error) -> Self {
        Error::Surreal(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
