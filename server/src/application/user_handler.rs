use rocket::http::hyper::StatusCode;
use rocket::response::status::{Accepted, Created};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

use crate::infrastructure::responses::{Error, ErrorResponse};
use crate::model::user_service;
use crate::DbConnection;

/// Handles the creation of a new user.
///
/// # Returns
/// A Result type:
/// * 201 and the id and username of the recently created user.
/// * 400 for any exception in the creation of the user, with specific
/// description.
/// * 500 for any other error.
#[post("/", format = "application/json", data = "<new_user_dto>")]
pub fn create_user(
    conn: DbConnection,
    new_user_dto: Json<UserDto>,
) -> Result<Created<Json<UserDto>>, Error> {
    let result = user_service::create_user(
        conn.borrow(),
        new_user_dto.username.to_string(),
        new_user_dto.password.to_string(),
    );
    match result {
        Ok(msg) => {
            println!("Created the username {}", new_user_dto.username);
            let dto = UserDto {
                id: Option::from(msg),
                username: new_user_dto.username.to_string(),
                password: "".to_string(),
            };
            Ok(Created(format!("/user/{}", msg), Option::from(Json(dto))))
        }
        Err(err) => {
            let err_msg = format!(
                "Cannot insert the username {} because {}",
                new_user_dto.username.to_string(),
                err
            );
            print!("{}", err_msg);
            Err(ErrorResponse::create_error(
                &err_msg,
                StatusCode::BadRequest,
            ))
        }
    }
}

#[post("/", format = "application/json", data = "<user_dto>")]
pub fn login(
    conn: DbConnection,
    user_dto: Json<UserDto>,
) -> Result<Accepted<Json<LoginDto>>, Error> {
    let new_login_result = user_service::login(
        conn.borrow(),
        user_dto.username.to_string(),
        user_dto.password.to_string(),
    );
    match new_login_result {
        Ok(login) => {
            let dto = LoginDto {
                token: login.get_token(),
                id: login.get_id(),
            };
            Ok(Accepted(Option::from(Json(dto))))
        }
        Err(_) => {
            let err_msg = String::from("Invalid credentials");
            print!("{}", err_msg);
            Err(ErrorResponse::create_error(
                &err_msg,
                StatusCode::BadRequest,
            ))
        }
    }
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
