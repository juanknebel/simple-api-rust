use rocket::{http::hyper::StatusCode, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::collections::HashMap;

pub type GenericResponse = HashMap<String, String>;

#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
  pub message: String,
}

impl ErrorResponse {
  pub fn create_error(message: &str, http_status_code: StatusCode) -> Error {
    match http_status_code {
      StatusCode::BadRequest => Error::BadRequestError(Json(ErrorResponse {
        message: message.to_string(),
      })),

      StatusCode::Unauthorized => {
        Error::UnauthorizedError(Json(ErrorResponse {
          message: message.to_string(),
        }))
      },

      _ => Error::StandardError(Json(ErrorResponse {
        message: message.to_string(),
      })),
    }
  }
}

#[derive(Debug, Responder)]
pub enum Error {
  #[response(status = 400, content_type = "application/json")]
  BadRequestError(Json<ErrorResponse>),
  #[response(status = 401, content_type = "application/json")]
  UnauthorizedError(Json<ErrorResponse>),
  #[response(status = 500, content_type = "application/json")]
  StandardError(Json<ErrorResponse>),
}
