use rocket::{http::hyper::StatusCode, response::status::Accepted, State};

use crate::{
  application::error::{ApplicationResult, ErrorResponse},
  UserService,
};

/// Implements a pong end point.
///
/// # Return
/// * 200 and pong message if we can make a simple sql query.
/// * 500 and the error message.
#[get("/ping")]
pub fn ping(
  us_state: State<Box<dyn UserService>>,
) -> ApplicationResult<Accepted<String>> {
  let user_service = us_state.inner();
  let result = user_service.total();

  match result {
    Ok(_) => Ok(Accepted(Option::from(String::from("pong")))),
    Err(err) => Err(ErrorResponse::create_error(
      &err.to_string(),
      StatusCode::InternalServerError,
    )),
  }
}
