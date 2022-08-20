use crate::{
  application::error::{ApplicationResult, ErrorResponse},
  UserService,
};

use rocket::{http::hyper::StatusCode, response::status::Accepted, State};

/// Implements a pong end point.
///
/// # Arguments
/// * `us_state` - The user service to check if the database is ok.
///
/// # Return
/// * 202 and pong message if we can make a simple sql query.
/// * 500 and the error message.
#[utoipa::path(
responses(
  (status = 202, description = "The server is ok"),
  (status = 500, description = "The server is malfunction")
),
)]
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::user_service::MockUserService;
  use rocket::{http::Status, local::Client};

  #[test]
  fn ping_ok() {
    let mut mock_us = MockUserService::new();
    mock_us.expect_total().times(1).returning(|| Ok(1));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .mount("/", routes![ping,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.body_string(), Some(String::from("pong")))
  }

  #[test]
  fn ping_fail() {
    let mut mock_us = MockUserService::new();
    mock_us
      .expect_total()
      .times(1)
      .returning(|| Err(String::from("some error")));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .mount("/", routes![ping,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"message\":\"some error\"}"))
    )
  }
}
