use std::fmt;
use std::fmt::Formatter;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use crate::DbConnection;


use diesel::dsl::count;
use diesel::dsl::count_star;
use diesel::query_dsl::QueryDsl;
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use diesel::types::{Bigint, Integer};
use crate::schema::users::dsl::users;
use crate::schema::users::id;

pub struct AccessToken(String);

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn is_valid(token: &str) -> bool {
    token == "ble"
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
        let conn = request.guard::<DbConnection>().unwrap();
        /*let result: QueryResult<BigInt> =
            users.select(count_star()).get_result(&*conn);
        match result {
            Ok(size) => println!("Hay usuarios"),
            Err(err) => println!("Hubo un error"),
        }

         */
        //println!("Hay {} usuarios", result);
        match tokens.len() {
            0 => Outcome::Failure((Status::BadRequest, AccessTokenError::Missing)),
            1 if is_valid(tokens[0]) => Outcome::Success(AccessToken(tokens[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, AccessTokenError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, AccessTokenError::BadCount)),
        }
    }
}
