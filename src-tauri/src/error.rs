#![allow(unused)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
