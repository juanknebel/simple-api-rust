use rocket::http::hyper::StatusCode;
use std::borrow::Borrow;

use crate::auth::middleware::AccessToken;
use rocket::{
  response::status::{Accepted, Created},
  State,
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
  application::error::{ApplicationResult, ErrorResponse, GenericResponse},
  Authenticator, MessageService,
};

/// Send a message from one user to another one.
///
/// # Arguments
/// * `msg_state` - The message service.
/// * `auth_state` - The authenticator used to validate the access token.
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `msg_dto` - The message dto to persist.
///
/// # Return
/// * 201 Created and the id of the message inserted.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid.
#[post("/send", format = "application/json", data = "<msg_dto>")]
pub fn send_message(
  msg_state: State<Box<dyn MessageService>>,
  auth_state: State<Box<dyn Authenticator>>,
  token: AccessToken,
  msg_dto: Json<MessageDto>,
) -> ApplicationResult<Created<Json<GenericResponse>>> {
  let message_service = msg_state.inner();
  let authenticator = auth_state.inner();
  match authenticator.authorize(token.borrow(), msg_dto.from) {
    Ok(_) => {
      let msg_id = message_service
        .create(
          msg_dto.from,
          msg_dto.to.unwrap(),
          msg_dto.message.as_ref().unwrap().to_string(),
        )
        .map_err(|err| {
          log::error!("error: {}", err.to_string());
          let err_msg = format!("Cannot insert the message because {}", err);
          ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
        })?;

      let mut response = GenericResponse::new();
      response.insert(String::from("id"), msg_id.to_string());
      Ok(Created(
        format!("/message/{}", msg_id),
        Option::from(Json(response)),
      ))
    },
    Err(_) => Err(ErrorResponse::create_error(
      "Access denied",
      StatusCode::Unauthorized,
    )),
  }
}

/// Get a message from its id.
///
/// # Arguments
/// * `msg_state` - The message service.
/// * `auth_state` - The authenticator used to validate the access token.
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `id` - The message id to retrieve.
///
/// # Return
/// * 202 Accepted and the message.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid (Not implemented yet).
#[get("/<id>", format = "application/json")]
pub fn get_message(
  msg_state: State<Box<dyn MessageService>>,
  _auth_state: State<Box<dyn Authenticator>>,
  _token: AccessToken,
  id: i32,
) -> ApplicationResult<Accepted<Json<ResponseMessageDto>>> {
  let message_service = msg_state.inner();
  let msg = message_service.get(id).map_err(|err| {
    log::error!("error: {}", err.to_string());
    let err_msg = format!("Cannot retrieve the message because {}", err);
    ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
  })?;

  let dto = ResponseMessageDto {
    id: None,
    to: None,
    message: msg.get_message(),
  };
  Ok(Accepted(Option::from(Json(dto))))
}

/// Get a message from the user specified, since the id indicated and with a
/// limit.
///
/// # Arguments
/// * `msg_state` - The message service.
/// * `auth_state` - The authenticator used to validate the access token.
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `msg_dto` - The message params to retrieve.
///
/// # Return
/// * 202 Accepted and the a list of messages order by id in desc mode.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid.
#[post("/", format = "application/json", data = "<msg_dto>")]
pub fn get_message_from(
  msg_state: State<Box<dyn MessageService>>,
  auth_state: State<Box<dyn Authenticator>>,
  token: AccessToken,
  msg_dto: Json<MessageDto>,
) -> ApplicationResult<Accepted<Json<Vec<ResponseMessageDto>>>> {
  let message_service = msg_state.inner();
  let authenticator = auth_state.inner();
  match authenticator.authorize(token.borrow(), msg_dto.from) {
    Ok(_) => {
      let messages = message_service
        .find(msg_dto.id.unwrap(), msg_dto.from, msg_dto.limit)
        .map_err(|err| {
          log::error!("error: {}", err.to_string());
          let err_msg = format!("Cannot retrieve the messages because {}", err);
          ErrorResponse::create_error(&err_msg, StatusCode::BadRequest)
        })?;

      let messages_dto = messages
        .iter()
        .map(|a_msg| ResponseMessageDto {
          id: Option::from(a_msg.get_id()),
          to: Option::from(a_msg.get_to()),
          message: a_msg.get_message(),
        })
        .collect::<Vec<ResponseMessageDto>>();
      Ok(Accepted(Option::from(Json(messages_dto))))
    },
    Err(_) => {
      log::error!("error: Access denied");
      Err(ErrorResponse::create_error(
        "Access denied",
        StatusCode::Unauthorized,
      ))
    },
  }
}

#[derive(Deserialize)]
pub struct MessageDto {
  id: Option<i32>,
  from: i32,
  to: Option<i32>,
  message: Option<String>,
  limit: Option<i64>,
}

#[derive(Serialize)]
pub struct ResponseMessageDto {
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  to: Option<i32>,
  message: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    auth::{error::Error::NoPermissionError, token::MockAuthenticator},
    model::{message::Builder, message_service::MockMessageService},
  };
  use mockall::predicate::{always, eq};
  use rocket::{
    http::{ContentType, Header, Status},
    local::Client,
  };

  #[test]
  fn send_message_ok() {
    let mut mock_ms = MockMessageService::new();
    mock_ms
      .expect_create()
      .with(eq(1), eq(2), eq(String::from("test message")))
      .times(1)
      .returning(|_, _, _| Ok(1));
    let mut mock_auth = MockAuthenticator::new();
    mock_auth
      .expect_authorize()
      .with(always(), eq(1))
      .times(1)
      .returning(|_, _| Ok(()));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_ms) as Box<dyn MessageService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
      .mount("/message", routes![send_message,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let access_token_header = Header::new("x-access-token", "Bearer 1");
    let mut request = client
      .post("/message/send")
      .body(r#"{ "from": 1, "to": 2, "message": "test message"}"#);
    request.add_header(ContentType::JSON);
    request.add_header(access_token_header);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.body_string(), Some(String::from("{\"id\":\"1\"}")))
  }

  #[test]
  fn send_message_unauthorized() {
    let mut mock_ms = MockMessageService::new();
    mock_ms
      .expect_create()
      .with(always(), always(), always())
      .times(0)
      .returning(|_, _, _| Ok(1));
    let mut mock_auth = MockAuthenticator::new();
    mock_auth
      .expect_authorize()
      .with(always(), eq(1))
      .times(1)
      .returning(|_, _| Err(NoPermissionError));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_ms) as Box<dyn MessageService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
      .mount("/message", routes![send_message,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let access_token_header = Header::new("x-access-token", "Bearer 1");
    let mut request = client
      .post("/message/send")
      .body(r#"{ "from": 1, "to": 2, "message": "test message"}"#);
    request.add_header(ContentType::JSON);
    request.add_header(access_token_header);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"message\":\"Access denied\"}"))
    )
  }

  #[test]
  fn send_message_without_access_token() {
    let mut mock_ms = MockMessageService::new();
    mock_ms
      .expect_create()
      .with(always(), always(), always())
      .times(0)
      .returning(|_, _, _| Ok(1));
    let mut mock_auth = MockAuthenticator::new();
    mock_auth.expect_authorize().with(always(), eq(1)).times(0);

    let rocket = rocket::ignite()
      .manage(Box::new(mock_ms) as Box<dyn MessageService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
      .mount("/message", routes![send_message,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let response = client
      .post("/message/send")
      .body(r#"{ "from": 1, "to": 2, "message": "test message"}"#)
      .header(ContentType::JSON)
      .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
  }

  #[test]
  fn get_message_ok() {
    let message = Builder::new()
      .with_id(1)
      .with_from(1)
      .with_to(2)
      .with_message("Some message")
      .build();

    let mut mock_ms = MockMessageService::new();
    mock_ms
      .expect_get()
      .with(eq(1))
      .times(1)
      .returning(move |_| Ok(message.clone()));
    let mut mock_auth = MockAuthenticator::new();
    mock_auth
      .expect_authorize()
      .with(always(), eq(1))
      .times(0)
      .returning(|_, _| Ok(()));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_ms) as Box<dyn MessageService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
      .mount("/message", routes![get_message,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let access_token_header = Header::new("x-access-token", "Bearer 1");
    let mut request = client.get("/message/1");
    request.add_header(ContentType::JSON);
    request.add_header(access_token_header);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(
      response.body_string(),
      Some(String::from("{\"message\":\"Some message\"}"))
    )
  }

  #[test]
  fn get_message_non_existing() {
    let mut mock_ms = MockMessageService::new();
    mock_ms
      .expect_get()
      .with(eq(1))
      .times(1)
      .returning(|_| Err(String::from("some error")));
    let mut mock_auth = MockAuthenticator::new();
    mock_auth
      .expect_authorize()
      .with(always(), eq(0))
      .times(0)
      .returning(|_, _| Ok(()));

    let rocket = rocket::ignite()
      .manage(Box::new(mock_ms) as Box<dyn MessageService>)
      .manage(Box::new(mock_auth) as Box<dyn Authenticator>)
      .mount("/message", routes![get_message,]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let access_token_header = Header::new("x-access-token", "Bearer 1");
    let mut request = client.get("/message/1");
    request.add_header(ContentType::JSON);
    request.add_header(access_token_header);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
      response.body_string(),
      Some(String::from(
        "{\"message\":\"Cannot retrieve the message because some error\"}"
      ))
    )
  }

  #[test]
  fn get_message_from_ok() {}

  #[test]
  fn get_message_from_ok_empty_messages() {}
}
