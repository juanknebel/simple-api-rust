use crate::{
  application::error::{ApplicationResult, ErrorResponse},
  Authenticator, UserService,
};

use rocket::{
  http::hyper::StatusCode,
  response::status::{Accepted, Created},
  State,
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use utoipa::Component;

/// Handles the creation of a new user.
///
/// # Arguments
/// * `us_state` - The user service.
/// * `user_dto` - The new user to be created.
///
/// # Return
/// A Result type:
/// * 201 Created and the id and username of the recently created user.
/// * 400 Bad request for any exception in the creation of the user, with
///   specific
/// description.
/// * 500 Internal error for any other error.
#[utoipa::path(
context_path = "/users",
request_body = UserDto,
responses(
(status = 201, description = "The user was created", body = ResponseUserDto),
(status = 400, description = "Bad request"),
(status = 500, description = "Internal error")
),
)]
#[post("/", format = "application/json", data = "<new_user_dto>")]
pub fn create_user(
  us_state: State<Box<dyn UserService>>,
  new_user_dto: Json<UserDto>,
) -> ApplicationResult<Created<Json<ResponseUserDto>>> {
  let user_service = us_state.inner();

  let id_user = user_service
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
  let dto = ResponseUserDto {
    id: id_user,
    username: new_user_dto.username.to_string(),
  };
  Ok(Created(
    format!("/user/{}", id_user),
    Option::from(Json(dto)),
  ))
}

/// Login a user. Checks if the username exist and if the password is the same.
/// This login generates a Jason Web Token with a expiration of 1 day.
/// If already exists another session for the user the a new token is generated
/// and replace the old one.
///
/// # Arguments
/// * `us_state` - The user service.
/// * `auth_state` - The authenticator used to validate the access token.
/// * `user_dto` - The user data to make the login.
///
/// # Return
/// * 202 Accepted and the Jason Web Token (JWT).
/// * 400 Bad request and the error message.
#[utoipa::path(
context_path = "/login",
request_body = UserDto,
responses(
(status = 202, description = "Login correct", body = LoginDto),
(status = 400, description = "Bad request")
),
)]
#[post("/", format = "application/json", data = "<user_dto>")]
pub fn login(
  us_state: State<Box<dyn UserService>>,
  auth_state: State<Box<dyn Authenticator>>,
  user_dto: Json<UserDto>,
) -> ApplicationResult<Accepted<Json<LoginDto>>> {
  let user_service = us_state.inner();
  let authenticator = auth_state.inner();

  let user = user_service
    .find_user(user_dto.username.to_string(), user_dto.password.to_string())
    .map_err(|err| {
      log::debug!("{}", err.to_string());
      let err_msg = String::from("Invalid credentials");
      ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
    })?;
  let token = authenticator.create_token(user.get_id()).map_err(|err| {
    log::debug!("{}", err.to_string());
    let err_msg = String::from("Cannot create the token");
    ErrorResponse::create_error(&err_msg, StatusCode::InternalServerError)
  })?;
  let login = user_service.login(user.borrow(), token).map_err(|err| {
    log::debug!("{}", err.to_string());
    let err_msg = String::from("Cannot make the login");
    ErrorResponse::create_error(&err_msg, StatusCode::InternalServerError)
  })?;

  let dto = LoginDto {
    token: login.get_token(),
    id: login.get_id(),
  };
  Ok(Accepted(Option::from(Json(dto))))
}

#[derive(Deserialize, Component)]
#[component(example = json!({"username": "juan", "password": "password"}))]
pub struct UserDto {
  username: String,
  password: String,
}

#[derive(Serialize, Component)]
#[component(example = json!({"id": 1, "username": "juan"}))]
pub struct ResponseUserDto {
  id: i32,
  username: String,
}

#[derive(Serialize, Component)]
#[component(example = json!({"id": 1, "token": "xxx"}))]
pub struct LoginDto {
  id: i32,
  token: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    auth::token::MockAuthenticator,
    model::{
      login::Builder, user::Builder as UserBuilder,
      user_service::MockUserService,
    },
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

    let user = UserBuilder::new()
      .with_id(1)
      .with_username("juan")
      .with_hashed_password("password")
      .build();
    mock_us
      .expect_find_user()
      .with(eq(String::from("juan")), eq(String::from("password")))
      .times(1)
      .returning(move |_, _| Ok(user.clone()));

    let login = Builder::new()
      .with_id(1)
      .with_username("juan")
      .with_token("my_token")
      .build();
    mock_us
      .expect_login()
      .with(always(), eq(String::from("my_token")))
      .times(1)
      .returning(move |_, _| Ok(login.clone()));

    let mut mock_auth = MockAuthenticator::new();
    mock_auth
      .expect_create_token()
      .with(eq(1))
      .times(1)
      .returning(|_| Ok("my_token".to_string()));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
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
      Some(String::from("{\"id\":1,\"token\":\"my_token\"}"))
    )
  }

  #[test]
  fn login_fail() {
    let mut mock_us = MockUserService::new();
    mock_us
      .expect_find_user()
      .with(eq(String::from("juan")), eq(String::from("password")))
      .times(1)
      .returning(move |_, _| Err(String::from("invalid password")));
    mock_us.expect_login().times(0);

    let mut mock_auth = MockAuthenticator::new();
    mock_auth.expect_create_token().times(0);

    let rocket = rocket::ignite()
      .manage(Box::new(mock_us) as Box<dyn UserService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
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
