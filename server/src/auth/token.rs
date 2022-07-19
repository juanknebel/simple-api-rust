use crate::auth::middleware::AccessToken;
use chrono::prelude::*;
use jsonwebtoken::{
  decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Formatter};

use crate::infrastructure::error::Error;

const BEARER: &str = "Bearer ";

#[derive(Debug)]
pub struct JwtConfig {
  secret: String,
}

impl JwtConfig {
  pub fn new(the_secret: String) -> JwtConfig {
    return JwtConfig {
      secret: the_secret,
    };
  }
}

impl fmt::Display for JwtConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.secret)
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
  sub: i32,
  exp: usize,
}

pub fn create_jwt(uid: i32, jwt_config: &JwtConfig) -> Result<String, Error> {
  let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::days(1))
    .expect("valid timestamp")
    .timestamp();

  let claims = Claims {
    sub: uid.to_owned(),
    exp: expiration as usize,
  };
  let header = Header::new(Algorithm::HS512);
  let token_result = encode(
    &header,
    &claims,
    &EncodingKey::from_secret(jwt_config.secret.as_ref()),
  )
  .map_err(|_| Error::JWTTokenCreationError);
  token_result
}

pub fn authorize(
  token: &AccessToken,
  uid: i32,
  jwt_config: &JwtConfig,
) -> Result<(), Error> {
  let token_as_string = jwt_from_header(token)?;
  let decoded = decode::<Claims>(
    &token_as_string,
    &DecodingKey::from_secret(jwt_config.secret.as_ref()),
    &Validation::new(Algorithm::HS512),
  )
  .map_err(|_| Error::JWTTokenError)?;

  if uid != decoded.claims.sub {
    return Err(Error::NoPermissionError);
  }
  Ok(())
}

fn jwt_from_header(token: &AccessToken) -> Result<String, Error> {
  if !token.get_token().starts_with(BEARER) {
    return Err(Error::InvalidAuthHeaderError);
  }
  Ok(token.get_token().trim_start_matches(BEARER).to_string())
}
