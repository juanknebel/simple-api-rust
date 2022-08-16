pub type RepoResult<T> = Result<T, diesel::result::Error>;

// use thiserror::Error;
// #[derive(Error, Debug)]
// pub enum Error {
// #[error("cannot open database connection")]
// ConnectionError,
// #[error("diesel error")]
// DieselError(diesel::result::Error),
// }
