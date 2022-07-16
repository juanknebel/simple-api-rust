use std::{borrow::Borrow, ops::Deref};

use diesel::prelude::*;

use crate::{
  model::{
    message::{Message, NewMessage},
    repository::error::RepoResult,
  },
  schema::{
    messages,
    messages::{from, id},
  },
  DbConnection,
};

pub fn add(conn: &DbConnection, new_message: NewMessage) -> RepoResult<i32> {
  diesel::insert_into(messages::table)
    .values(new_message.borrow())
    .execute(conn.deref())?;
  let msg = find_latest_msg(conn.borrow(), new_message.get_from())?;
  Ok(msg.get_id())
}

fn find_latest_msg(conn: &DbConnection, from_user: i32) -> RepoResult<Message> {
  let msg = messages::table
    .filter(from.eq(from_user))
    .order(id.desc())
    .first(conn.deref())?;
  Ok(msg)
}

pub fn get(conn: &DbConnection, id_msg: i32) -> RepoResult<Message> {
  let msg = messages::table.find(id_msg).get_result(conn.deref())?;
  Ok(msg)
}

pub fn find(
  conn: &DbConnection,
  from_msg: i32,
  from_user: i32,
  limit: i64,
) -> RepoResult<Vec<Message>> {
  let messages = messages::table
    .filter(id.ge(from_msg).and(from.eq(from_user)))
    .limit(limit)
    .order(id.desc())
    .load(conn.deref())?;
  Ok(messages)
}
