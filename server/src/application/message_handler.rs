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
  auth::token,
  model::message_service,
  DbConnection, JwtConfig,
};

/// Send a message from one user to another one.
///
/// # Arguments
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `jwt_config` - The jwt configuration used to validate the access token.
/// * `conn` - The database connection.
/// * `msg_dto` - The message dto to persist.
///
/// # Return
/// * 201 Created and the id of the message inserted.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid.
#[post("/send", format = "application/json", data = "<msg_dto>")]
pub fn send_message(
  token: AccessToken,
  jwt_config: State<JwtConfig>,
  conn: DbConnection,
  msg_dto: Json<MessageDto>,
) -> ApplicationResult<Created<Json<GenericResponse>>> {
  match is_valid(token.borrow(), jwt_config.inner(), msg_dto.from) {
    true => {
      let msg_id = message_service::create(
        conn.borrow(),
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
    false => Err(ErrorResponse::create_error(
      "Access denied",
      StatusCode::Unauthorized,
    )),
  }
}

/// Get a message from its id.
///
/// # Arguments
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `jwt_config` - The jwt configuration used to validate the access token.
/// * `conn` - The database connection.
/// * `id` - The message id to retrieve.
///
/// # Return
/// * 202 Accepted and the message.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid (Not implemented yet).
#[get("/<id>", format = "application/json")]
pub fn get_message(
  _token: AccessToken,
  _jwt_config: State<JwtConfig>,
  conn: DbConnection,
  id: i32,
) -> ApplicationResult<Accepted<Json<ResponseMessageDto>>> {
  let msg = message_service::get(conn.borrow(), id).map_err(|err| {
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
/// * `token` - The access token used to validate if the user who sent the
///   messages is a valid one.
/// * `jwt_config` - The jwt configuration used to validate the access token.
/// * `conn` - The database connection.
/// * `msg_dto` - The message params to retrieve.
///
/// # Return
/// * 202 Accepted and the a list of messages order by id in desc mode.
/// * 400 Bad request and the error message.
/// * 401 Unauthorized if the token isn't valid.
#[post("/", format = "application/json", data = "<msg_dto>")]
pub fn get_message_from(
  token: AccessToken,
  jwt_config: State<JwtConfig>,
  conn: DbConnection,
  msg_dto: Json<MessageDto>,
) -> ApplicationResult<Accepted<Json<Vec<ResponseMessageDto>>>> {
  match is_valid(token.borrow(), jwt_config.inner(), msg_dto.from) {
    true => {
      let messages = message_service::find(
        conn.borrow(),
        msg_dto.id.unwrap(),
        msg_dto.from,
        msg_dto.limit,
      )
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
    false => {
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

/// Validate if a token belongs to a specific user.
///
/// # Arguments
/// * `token` - The access token to be validated.
/// * `jwt_config` - The jwt configuration use to validate.
/// * `id_user` - The user_id to check if the token matches.
///
/// # Return
/// * True if the access token is valid and belongs to the user.
/// * False otherwise.
fn is_valid(token: &AccessToken, jwt_config: &JwtConfig, id_user: i32) -> bool {
  match token::authorize(token, id_user, jwt_config) {
    Ok(_) => true,
    Err(err) => {
      log::debug!("error: {}", err.to_string());
      false
    },
  }
}
