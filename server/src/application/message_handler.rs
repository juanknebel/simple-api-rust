use rocket::http::hyper::StatusCode;
use std::borrow::Borrow;

use crate::application::middleware::AccessToken;
use rocket::response::status::Created;
use rocket_contrib::json::Json;
use serde::Deserialize;

use crate::infrastructure::responses::{Error, ErrorResponse, GenericResponse};
use crate::model::message_service;
use crate::model::user_service;
use crate::DbConnection;

#[post("/send", format = "application/json", data = "<msg_dto>")]
pub fn send_message(
    token: AccessToken,
    conn: DbConnection,
    msg_dto: Json<MessageDto>,
) -> Result<Created<Json<GenericResponse>>, Error> {
    let can_access = is_valid(conn.borrow(), token.borrow(), msg_dto.from);
    match can_access {
        true => {
            let result = message_service::create_message(
                conn.borrow(),
                msg_dto.from,
                msg_dto.to,
                msg_dto.message.to_string(),
            );
            match result {
                Ok(msg_id) => {
                    let mut response = GenericResponse::new();
                    response.insert(String::from("id"), msg_id.to_string());
                    Ok(Created(
                        format!("/message/{}", msg_id),
                        Option::from(Json(response)),
                    ))
                }
                Err(err) => {
                    let err_msg = format!("Cannot insert the message because {}", err);
                    print!("{}", err_msg);
                    Err(ErrorResponse::create_error(
                        &err_msg,
                        StatusCode::BadRequest,
                    ))
                }
            }
        }
        false => Err(ErrorResponse::create_error(
            "Access denied",
            StatusCode::Unauthorized,
        )),
    }
}

#[derive(Deserialize)]
pub struct MessageDto {
    pub from: i32,
    pub to: i32,
    pub message: String,
}

fn is_valid(conn: &DbConnection, token: &AccessToken, id_user: i32) -> bool {
    let user_result = user_service::get(conn.borrow(), id_user);
    match user_result {
        Ok(user) => {
            let token_belong_to_user = user_service::is_same_token(
                conn.borrow(),
                token.get_token().as_str(),
                user.borrow(),
            );
            match token_belong_to_user {
                Ok(is_valid_token) => is_valid_token,
                Err(err) => {
                    println!("Error {}", err.to_string());
                    false
                }
            }
        }
        Err(err) => {
            println!("Error {}", err.to_string());
            false
        }
    }
}
