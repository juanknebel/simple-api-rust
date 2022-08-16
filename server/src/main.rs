#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod application;
mod auth;
mod db;
mod log;
mod model;
mod schema;

use crate::{
  auth::token::{setup_jwt_config, JwtConfig},
  db::database::{establish_connection, DbConnection},
  log::log::setup_logger,
  model::{
    message_service::{MessageService, MessageServiceImpl},
    repository::{
      login_repository::LoginRepositoryImpl,
      message_repository::MessageRepositoryImpl,
      user_repository::UserRepositoryImpl,
    },
    user_service::{UserService, UserServiceImpl},
  },
};
use application::{health_handler, message_handler, user_handler};
use rocket::routes;

fn main() {
  // Set up the logger
  setup_logger();

  // Bearer token configuration
  let jwt_config = setup_jwt_config();

  // Database pool
  let db_conn = DbConnection::new(establish_connection());

  // Repository initialization
  let user_repository = UserRepositoryImpl::new(db_conn.clone());
  let login_repository = LoginRepositoryImpl::new(db_conn.clone());
  let message_repository = MessageRepositoryImpl::new(db_conn.clone());

  // User related initialization
  let user_service = UserServiceImpl::new(user_repository, login_repository);

  // Messages related initialization
  let message_service = MessageServiceImpl::new(message_repository);

  rocket::Rocket::ignite()
    .manage(jwt_config)
    .manage(Box::new(user_service) as Box<dyn UserService>)
    .manage(Box::new(message_service) as Box<dyn MessageService>)
    .mount("/", routes![health_handler::ping,])
    .mount("/users", routes![user_handler::create_user,])
    .mount("/login", routes![user_handler::login,])
    .mount(
      "/message",
      routes![
        message_handler::send_message,
        message_handler::get_message,
        message_handler::get_message_from
      ],
    )
    .launch();
}
