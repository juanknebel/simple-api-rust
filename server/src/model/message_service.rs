use std::borrow::Borrow;

use crate::model::message::{Message, NewMessage};
use crate::model::repository::message_repository;
use crate::DbConnection;

pub fn create_message(
    conn: &DbConnection,
    from: i32,
    to: i32,
    message: String,
) -> Result<i32, String> {
    let new_message = NewMessage::new(from, to, message);
    message_repository::add(conn.borrow(), new_message)
}

pub fn get_message(conn: &DbConnection, id: i32) -> Result<Message, String> {
    message_repository::get(conn.borrow(), id)
}
