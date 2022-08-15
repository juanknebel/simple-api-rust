#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod application;
mod auth;
mod infrastructure;
mod model;
mod schema;

use crate::{
  auth::token::JwtConfig,
  model::{
    message_service::{MessageService, MessageServiceImpl},
    user_service::{UserService, UserServiceImpl},
  },
};
use application::{health_handler, message_handler, user_handler};
use rocket::{config::Environment, routes};
use rocket_contrib::databases::{database, diesel::SqliteConnection};

#[database("sqlite")]
pub struct DbConnection(SqliteConnection);

/// Setup the logger based on the environment in which it's been deploy.
/// If the environment is development then the level of logging is set to Trace.
/// In any other case it's set in Warning.
///
/// # Arguments
/// * `environment` - The environment in which the application is been deploy.
fn setup_logger(environment: Environment) {
  use log::LevelFilter;

  let level_filter;
  match environment {
    Environment::Development => level_filter = LevelFilter::Trace,
    _ => level_filter = LevelFilter::Warn,
  }

  let (level, logger) = fern::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "[{date}] [{level}][where: {target}, line: {line}] [{message}]",
        date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
        target = record.target(),
        level = record.level(),
        line = record.line().unwrap_or(0),
        message = message
      ))
    })
    .level(level_filter)
    .chain(std::io::stdout())
    .chain(
      fern::log_file("logs/application.log")
        .unwrap_or_else(|_| panic!("Cannot open logs/application.log")),
    )
    .into_log();
  async_log::Logger::wrap(logger, || 0).start(level).unwrap();
}

/// Initialize the JwtConfig for the entire application.
///
/// # Arguments
/// * `jwt_secret` - The secret use to encode and decode all the jason web
///   tokens.
///
/// # Return
/// * A new JwtConfig.
pub fn setup_jwtconfig(jwt_secret: String) -> JwtConfig {
  JwtConfig::new(jwt_secret)
}

fn main() {
  setup_logger(rocket::Config::active().unwrap().environment);
  let rocket = rocket::Rocket::ignite();
  let jwt_config = setup_jwtconfig(
    rocket
      .config()
      .extras
      .get("jwt_secret")
      .unwrap()
      .to_string(),
  );

  rocket
    .attach(DbConnection::fairing())
    .manage(jwt_config)
    .manage(Box::new(UserServiceImpl) as Box<dyn UserService>)
    .manage(Box::new(MessageServiceImpl) as Box<dyn MessageService>)
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
