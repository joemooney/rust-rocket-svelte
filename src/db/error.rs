use thiserror::Error;

pub type DbResult<T> = std::result::Result<T, DbError>;


#[derive(Error, Debug)]
pub enum DbError {
    /// a error trying to save server JSON state
    #[error("JSON db save error: {0}")]
    SaveError(String),
    /// a error trying to load server JSON state
    #[error("JSON db load error: {0}")]
    LoadError(String),
}