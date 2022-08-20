use rocket::{
  http::{Header, Status},
  Response, State,
};
use std::{io::Cursor, path::PathBuf, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

use crate::{
  application::{health_handler, message_handler, user_handler},
  message_handler::{MessageDto, ResponseMessageDto, SearchMessageDto},
  user_handler::{LoginDto, ResponseUserDto, UserDto},
};

#[derive(OpenApi)]
#[openapi(
  handlers(
    health_handler::ping,
    message_handler::send_message,
    message_handler::get_message,
    message_handler::get_message_from,
    user_handler::create_user,
    user_handler::login,
  ),
  components(
    MessageDto,
    ResponseMessageDto,
    SearchMessageDto,
    UserDto,
    ResponseUserDto,
    LoginDto
  )
)]
pub struct ApiDoc;

#[get("/<tail..>")]
pub fn serve_swagger(
  tail: PathBuf,
  config: State<Arc<Config>>,
) -> Response<'static> {
  match utoipa_swagger_ui::serve(
    tail.as_os_str().to_str().unwrap(),
    config.clone(),
  ) {
    Ok(file) => file
      .map(|file| {
        Response::build()
          .sized_body(Cursor::new(file.bytes.to_vec()))
          .header(Header::new("Content-Type", file.content_type))
          .finalize()
      })
      .unwrap_or_else(|| Response::build().status(Status::NotFound).finalize()),
    Err(error) => {
      let error = error.to_string();
      let len = error.len() as u64;

      Response::build()
        .raw_body(rocket::response::Body::Sized(Cursor::new(error), len))
        .status(Status::InternalServerError)
        .finalize()
    },
  }
}

#[get("/api-doc/openapi.json")]
pub fn serve_api_doc(
  openapi: State<utoipa::openapi::OpenApi>,
) -> Response<'static> {
  let json_string = serde_json::to_string(openapi.inner()).unwrap();
  let len = json_string.len() as u64;

  Response::build()
    .raw_body(rocket::response::Body::Sized(Cursor::new(json_string), len))
    .header(Header::new("Content-Type", "application/json"))
    .finalize()
}
