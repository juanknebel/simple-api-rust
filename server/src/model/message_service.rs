use std::borrow::Borrow;

use crate::{
  model::{
    error::ServiceResult,
    message::{Message, NewMessage},
    repository::message_repository,
  },
  DbConnection,
};

pub fn create(
  conn: &DbConnection,
  from: i32,
  to: i32,
  message: String,
) -> ServiceResult<i32> {
  let new_message = NewMessage::new(from, to, message);
  message_repository::add(conn.borrow(), new_message)
    .map_err(|err| err.to_string())
}

pub fn get(conn: &DbConnection, id: i32) -> ServiceResult<Message> {
  message_repository::get(conn.borrow(), id).map_err(|err| err.to_string())
}

pub fn find(
  conn: &DbConnection,
  from_msg: i32,
  from_user: i32,
  limit: i64,
) -> ServiceResult<Vec<Message>> {
  message_repository::find(conn.borrow(), from_msg, from_user, limit)
    .map_err(|err| err.to_string())
}
