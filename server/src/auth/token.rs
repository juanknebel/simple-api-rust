use crate::auth::middleware::AccessToken;
use chrono::prelude::*;
use jsonwebtoken::{
  decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Formatter};

use crate::auth::error::{AuthResult, Error};

use dotenv::dotenv;
use std::env;

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

/// Create a Jason Web Token, based on a uid using the HS512 Algorithm.
///
/// # Arguments
/// * `uid` - The uid of the entity that needs a token.
/// * `jwt_config` - The jwt configuration to generate the web token.
///
/// # Return
/// * A string that represents the Jason Web Token.
/// * A JWTTokenCreationError in case of failed.
pub fn create_jwt(uid: i32, jwt_config: &JwtConfig) -> AuthResult<String> {
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

/// Authorize an uid if the access token is valid and belongs to the uid.
///
/// # Arguments
/// * `token` - The access token to validate. Must be in the Bearer form.
/// * `uid` - The uid to check if it is the same as the access token.
/// * `jwt_config` - The jwt configuration to validate the web token.
///
/// # Return
/// * Nothing if the validation was successful.
/// * JWTTokenError if an error occur in the decode process.
/// * NoPermissionError if the token doesn't belong to the uid.
pub fn authorize(
  token: &AccessToken,
  uid: i32,
  jwt_config: &JwtConfig,
) -> AuthResult<()> {
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

/// Extract the value of token from the string in the AccessToken.
/// The AccessToken must be 'BEARER xxxxx'.
///
/// # Arguments
/// * `token` - The AccessToken to extract the value.
///
/// # Return
/// * A string that represents the value of the JWT.
/// * InvalidAuthHeaderError if the header doesn't respect the specification.
fn jwt_from_header(token: &AccessToken) -> AuthResult<String> {
  if !token.get_token().starts_with(BEARER) {
    return Err(Error::InvalidAuthHeaderError);
  }
  Ok(token.get_token().trim_start_matches(BEARER).to_string())
}

/// Initialize the JwtConfig for the entire application.
///
/// # Arguments
/// * `jwt_secret` - The secret use to encode and decode all the jason web
///   tokens.
///
/// # Return
/// * A new JwtConfig.
pub fn setup_jwt_config() -> JwtConfig {
  if cfg!(test) {
    JwtConfig::new("secret".to_string())
  } else {
    dotenv().ok();

    let jwt_secret = env::var("jwt_secret").expect("jwt_secret must be set");
    JwtConfig::new(jwt_secret)
  }
}
