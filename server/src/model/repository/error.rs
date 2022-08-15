use thiserror::Error;

pub type RepoResult<T> = Result<T, diesel::result::Error>;

#[derive(Error, Debug)]
pub enum Error {
  #[error("cannot open database connection")]
  ConnectionError,
  #[error("diesel error")]
  DieselError(diesel::result::Error),
}
