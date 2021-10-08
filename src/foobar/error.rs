use thiserror::Error;

pub type FoobarResult<T> = std::result::Result<T, FoobarError>;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum FoobarError {
    /// A error trying to bind to a UDP port
    #[error("udp event listener bind failure")]
    BindError { source: std::io::Error }
}
