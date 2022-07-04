use rocket::http::hyper::StatusCode;

use rocket::response::status::Created;
use rocket_contrib::json::Json;
use serde::Deserialize;
use crate::application::middleware::AccessToken;

use crate::infrastructure::responses::{Error, ErrorResponse, GenericResponse};
use crate::DbConnection;

#[post("/send", format = "application/json", data = "<msg_dto>")]
pub fn send_message(
    token: AccessToken,
    conn: DbConnection,
    msg_dto: Json<MessageDto>,
) -> Result<Created<Json<GenericResponse>>, Error> {
    println!("the token is {}", token);
    let mut response = GenericResponse::new();
    response.insert("id", "1");
    Ok(Created(
        format!("/messages/user/{}", msg_dto.to),
        Option::from(Json(response)),
    ))
}

#[derive(Deserialize)]
pub struct MessageDto {
    from: i32,
    to: i32,
    message: String,
}
