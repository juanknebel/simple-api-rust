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

use application::{health_handler, message_handler, user_handler};
use rocket::{config::Environment, routes};
use rocket_contrib::databases::{database, diesel::SqliteConnection};

#[database("sqlite")]
pub struct DbConnection(SqliteConnection);

fn setup_logger(env: Environment) {
  use log::LevelFilter;

  let mut level_filter = LevelFilter::Warn;
  if env.is_dev() {
    level_filter = LevelFilter::Trace;
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

fn main() {
  setup_logger(rocket::Config::active().unwrap().environment);
  rocket::Rocket::ignite()
    .attach(DbConnection::fairing())
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
