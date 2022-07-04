use std::fmt;
use std::fmt::Formatter;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

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

fn is_valid(token: &str) -> bool {
    token.len() > 0
}

#[derive(Debug)]
pub enum AccessTokenError {
    Invalid,
    BadCount,
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = AccessTokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let tokens: Vec<&str> = request.headers().get("x-access-token").collect();
        let route = request.route().unwrap();
        println!("{}", route.name.unwrap());
        let uri = request.uri();
        println!("{}", uri.path());

        match tokens.len() {
            0 => Outcome::Failure((Status::BadRequest, AccessTokenError::Missing)),
            1 if is_valid(tokens[0]) => Outcome::Success(AccessToken(tokens[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, AccessTokenError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, AccessTokenError::BadCount)),
        }
    }
}
