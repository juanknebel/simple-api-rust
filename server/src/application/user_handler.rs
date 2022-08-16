use rocket::{
  http::hyper::StatusCode,
  response::status::{Accepted, Created},
  State,
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
  application::error::{ApplicationResult, ErrorResponse},
  JwtConfig, UserService,
};

/// Handles the creation of a new user.
///
/// # Return
/// A Result type:
/// * 201 Created and the id and username of the recently created user.
/// * 400 Bad request for any exception in the creation of the user, with
///   specific
/// description.
/// * 500 Internal error for any other error.
#[post("/", format = "application/json", data = "<new_user_dto>")]
pub fn create_user(
  us_state: State<Box<dyn UserService>>,
  new_user_dto: Json<UserDto>,
) -> ApplicationResult<Created<Json<UserDto>>> {
  let user_service = us_state.inner();

  let msg = user_service
    .create_user(
      new_user_dto.username.to_string(),
      new_user_dto.password.to_string(),
    )
    .map_err(|err| {
      let err_msg = format!(
        "Cannot insert the username {} because {}",
        new_user_dto.username.to_string(),
        err
      );
      log::error!("{}", err_msg);
      ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
    })?;

  log::info!("new username {}", new_user_dto.username);
  let dto = UserDto {
    id: Option::from(msg),
    username: new_user_dto.username.to_string(),
    password: "".to_string(),
  };
  Ok(Created(format!("/user/{}", msg), Option::from(Json(dto))))
}

/// Login a user. Checks if the username exist and if the password is the same.
/// This login generates a Jason Web Token with a expiration of 1 day.
/// If already exists another session for the user the a new token is generated
/// and replace the old one.
///
/// # Arguments
/// * `jwt_config` - The jwt configuration used to generate the access token.
/// * `user_dto` - The user data to make the login.
///
/// # Return
/// * 202 Accepted and the Jason Web Token (JWT).
/// * 400 Bad request and the error message.
#[post("/", format = "application/json", data = "<user_dto>")]
pub fn login(
  us_state: State<Box<dyn UserService>>,
  jwt_config: State<JwtConfig>,
  user_dto: Json<UserDto>,
) -> ApplicationResult<Accepted<Json<LoginDto>>> {
  let user_service = us_state.inner();
  let login = user_service
    .login(
      jwt_config.inner(),
      user_dto.username.to_string(),
      user_dto.password.to_string(),
    )
    .map_err(|err| {
      log::debug!("{}", err.to_string());
      let err_msg = String::from("Invalid credentials");
      ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
    })?;

  let dto = LoginDto {
    token: login.get_token(),
    id: login.get_id(),
  };
  Ok(Accepted(Option::from(Json(dto))))
}

#[derive(Deserialize, Serialize)]
pub struct UserDto {
  id: Option<i32>,
  username: String,
  #[serde(skip_serializing)]
  password: String,
}

#[derive(Serialize)]
pub struct LoginDto {
  id: i32,
  token: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    model::{login::Builder, user_service::MockUserService},
    setup_jwt_config,
  };
  use mockall::predicate::{always, eq};
  use rocket::{
    http::{ContentType, Status},
    local::Client,
  };

  #[test]
  fn create_user_ok() {
    let mut mock_us = MockUserService::new();
    mock_us
      .expect_create_user()
      .with(eq(String::from("juan")), eq(String::from("password")))
      .times(1)
      .returning(|_, _| Ok(1));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .mount("/users", routes![create_user,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client
      .post("/users")
      .body(r#"{ "username": "juan", "password": "password"}"#)
      .header(ContentType::JSON)
      .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"id\":1,\"username\":\"juan\"}"))
    )
  }

  #[test]
  fn create_user_fail() {
    let mut mock_us = MockUserService::new();
    mock_us
      .expect_create_user()
      .with(eq(String::from("juan")), eq(String::from("password")))
      .times(1)
      .returning(|_, _| Err(String::from("cannot create user")));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .mount("/users", routes![create_user,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client
      .post("/users")
      .body(r#"{ "username": "juan", "password": "password"}"#)
      .header(ContentType::JSON)
      .dispatch();
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
      response.body_string(),
      Some(String::from(
        "{\"message\":\"Cannot insert the username juan because cannot create \
         user\"}"
      ))
    )
  }

  #[test]
  fn login_ok() {
    let mut mock_us = MockUserService::new();
    let login = Builder::new()
      .with_id(1)
      .with_username("juan")
      .with_token("mytoken")
      .build();

    mock_us
      .expect_login()
      .with(
        always(),
        eq(String::from("juan")),
        eq(String::from("password")),
      )
      .times(1)
      .returning(move |_, _, _| Ok(login.clone()));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .manage(setup_jwt_config())
      .mount("/login", routes![login,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client
      .post("/login")
      .body(r#"{ "username": "juan", "password": "password"}"#)
      .header(ContentType::JSON)
      .dispatch();
    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"id\":1,\"token\":\"mytoken\"}"))
    )
  }

  #[test]
  fn login_fail() {
    let mut mock_us = MockUserService::new();
    mock_us
      .expect_login()
      .with(
        always(),
        eq(String::from("juan")),
        eq(String::from("password")),
      )
      .times(1)
      .returning(|_, _, _| Err(String::from("invalid password")));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .manage(setup_jwt_config())
      .mount("/login", routes![login,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let mut response = client
      .post("/login")
      .body(r#"{ "username": "juan", "password": "password"}"#)
      .header(ContentType::JSON)
      .dispatch();
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"message\":\"Invalid credentials\"}"))
    )
  }
}
