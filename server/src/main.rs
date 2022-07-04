#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod application;
mod infrastructure;
mod model;
mod schema;

use application::health_handler;
use application::message_handler;
use application::user_handler;
use rocket::routes;
use rocket_contrib::databases::{database, diesel::SqliteConnection};

#[database("sqlite")]
pub struct DbConnection(SqliteConnection);

fn main() {
    rocket::Rocket::ignite()
        .attach(DbConnection::fairing())
        .mount("/", routes![health_handler::ping,])
        .mount("/users", routes![user_handler::create_user,])
        .mount("/login", routes![user_handler::login,])
        .mount("/message", routes![message_handler::send_message,])
        .launch();
}
