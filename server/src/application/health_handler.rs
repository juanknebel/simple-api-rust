use diesel::prelude::*;
use rocket::http::hyper::StatusCode;
use rocket::response::status::Accepted;

use crate::infrastructure::responses::{Error, ErrorResponse};
use crate::schema::users;
use crate::schema::users::id;
use crate::DbConnection;

/// Implements a pong end point.
///
/// # Return
/// * 200 and pong message if we can make a simple sql query.
/// * 500 and the error message.
#[get("/ping")]
pub fn ping(conn: DbConnection) -> Result<Accepted<String>, Error> {
    let result: Result<usize, diesel::result::Error> = users::table
        .select(id).count().execute(&*conn);

    match result {
        Ok(_) => Ok(Accepted(Option::from(String::from("pong")))),
        Err(err) => Err(ErrorResponse::create_error(
            &err.to_string(),
            StatusCode::InternalServerError,
        )),
    }
}
