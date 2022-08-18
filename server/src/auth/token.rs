use crate::auth::middleware::AccessToken;
use chrono::prelude::*;
use jsonwebtoken::{
  decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::auth::error::{AuthResult, Error};

use dotenv::dotenv;
use std::env;

#[cfg(test)]
use mockall::automock;

const BEARER: &str = "Bearer ";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
  sub: i32,
  exp: usize,
}

#[cfg_attr(test, automock)]
pub trait Authenticator: Send + Sync {
  /// Create a Jason Web Token, based on a uid using the HS512 Algorithm.
  ///
  /// # Arguments
  /// * `uid` - The uid of the entity that needs a token.
  ///
  /// # Return
  /// * A string that represents the Jason Web Token.
  /// * A JWTTokenCreationError in case of failed.
  fn create_token(&self, uid: i32) -> AuthResult<String>;

  /// Authorize an uid if the access token is valid and belongs to the uid.
  ///
  /// # Arguments
  /// * `token` - The access token to validate. Must be in the Bearer form.
  /// * `uid` - The uid to check if it is the same as the access token.
  ///
  /// # Return
  /// * Nothing if the validation was successful.
  /// * JWTTokenError if an error occur in the decode process.
  /// * NoPermissionError if the token doesn't belong to the uid.
  fn authorize(&self, token: &AccessToken, uid: i32) -> AuthResult<()>;
}

#[derive(Debug)]
pub struct BearerAuthenticator {
  secret: String,
}

impl BearerAuthenticator {
  pub fn new() -> BearerAuthenticator {
    BearerAuthenticator {
      secret: setup_jwt_config(),
    }
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
  fn jwt_from_header(&self, token: &AccessToken) -> AuthResult<String> {
    if !token.get_token().starts_with(BEARER) {
      return Err(Error::InvalidAuthHeaderError);
    }
    Ok(token.get_token().trim_start_matches(BEARER).to_string())
  }
}

impl Authenticator for BearerAuthenticator {
  fn create_token(&self, uid: i32) -> AuthResult<String> {
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
      &EncodingKey::from_secret(self.secret.as_ref()),
    )
    .map_err(|_| Error::JWTTokenCreationError);
    token_result
  }

  fn authorize(&self, token: &AccessToken, uid: i32) -> AuthResult<()> {
    let token_as_string = self.jwt_from_header(token)?;
    let decoded = decode::<Claims>(
      &token_as_string,
      &DecodingKey::from_secret(self.secret.as_ref()),
      &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;

    if uid != decoded.claims.sub {
      return Err(Error::NoPermissionError);
    }
    Ok(())
  }
}

/// Initialize the JwtConfig for the entire application.
///
/// # Arguments
///
/// # Return
/// * A string that represents the secret for a JWT.
fn setup_jwt_config() -> String {
  if cfg!(test) {
    "secret".to_string()
  } else {
    dotenv().ok();

    env::var("jwt_secret").expect("jwt_secret must be set")
  }
}
