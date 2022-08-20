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
mod openapi;
mod schema;

use crate::{
  auth::token::{Authenticator, BearerAuthenticator},
  db::database::{establish_connection, DbConnection},
  log::log::setup_logger,
  model::{
    message_service::{MessageService, MessageServiceImpl},
    password::SimpleHasher,
    repository::{
      login_repository::LoginRepositoryImpl,
      message_repository::MessageRepositoryImpl,
      user_repository::UserRepositoryImpl,
    },
    user_service::{UserService, UserServiceImpl},
  },
  openapi::swagger,
};

use application::{health_handler, message_handler, user_handler};
use rocket::routes;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

fn main() {
  // Set up the logger
  setup_logger();

  // Bearer token configuration
  let authenticator = BearerAuthenticator::new();

  // Database pool
  let db_conn = DbConnection::new(establish_connection());

  // Repository initialization
  let user_repository = UserRepositoryImpl::new(db_conn.clone());
  let login_repository = LoginRepositoryImpl::new(db_conn.clone());
  let message_repository = MessageRepositoryImpl::new(db_conn.clone());

  // User related initialization
  let password_hasher = SimpleHasher::default();
  let user_service =
    UserServiceImpl::new(user_repository, login_repository, password_hasher);

  // Messages related initialization
  let message_service = MessageServiceImpl::new(message_repository);

  rocket::Rocket::ignite()
    .manage(Box::new(authenticator) as Box<dyn Authenticator>)
    .manage(Box::new(user_service) as Box<dyn UserService>)
    .manage(Box::new(message_service) as Box<dyn MessageService>)
    .manage(Arc::new(Config::from("/swagger/api-doc/openapi.json")))
    .manage(swagger::ApiDoc::openapi())
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
    .mount(
      "/swagger",
      routes![swagger::serve_api_doc, swagger::serve_swagger],
    )
    .launch();
}
