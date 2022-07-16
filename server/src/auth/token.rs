use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::infrastructure::error::Error;

//const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"my very super secret";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub fn create_jwt(uid: i32) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let token_result = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError);
    token_result
}
