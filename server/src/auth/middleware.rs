use std::{fmt, fmt::Formatter};

use rocket::{
  http::Status,
  request::{FromRequest, Outcome},
  Request,
};

pub struct AccessToken(String);

impl AccessToken {
  pub fn get_token(&self) -> String {
    self.0.to_string()
  }
}

impl fmt::Display for AccessToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
pub enum AccessTokenError {
  BadCount,
  Missing,
}

/// Implements the FromRequest trait to make the header x-access-token appear in
/// the guards of every endpoint.
///
/// # Return
/// * Success and an AccessToken struct if the header is present.
/// * Failure with AccessTokenError::Missing if there is no value for the
///   header.
/// * Failure with AccessTokenError::BasCount if there is more than one value
///   for the header.
impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
  type Error = AccessTokenError;

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    let tokens: Vec<&str> = request.headers().get("x-access-token").collect();
    let route = request.route().unwrap();
    let uri = request.uri();
    log::debug!("uri: {}, method: {}", uri.path(), route.name.unwrap());

    match tokens.len() {
      0 => Outcome::Failure((Status::BadRequest, AccessTokenError::Missing)),
      1 => Outcome::Success(AccessToken(tokens[0].to_string())),
      _ => Outcome::Failure((Status::BadRequest, AccessTokenError::BadCount)),
    }
  }
}
