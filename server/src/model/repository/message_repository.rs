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

pub trait MessageRepository {
  /// Insert a message in the database
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `new_message` - The new message to be inserted.
  ///
  /// # Return
  /// * The id of the message.
  /// * A diesel error.
  fn add(
    &self,
    conn: &DbConnection,
    new_message: NewMessage,
  ) -> RepoResult<i32>;

  /// Retrieve a message from its id.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `id_msg` - The id of the message to look for.
  ///
  /// # Return
  /// * The message struct.
  /// * A diesel error.
  fn get(&self, conn: &DbConnection, id_msg: i32) -> RepoResult<Message>;

  /// Look for messages based on the parameters. The messages are return in
  /// order descending by its ids.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `from_msg` - The id of the message from which start the search. Could
  ///   not
  /// be the id of a message from the user.
  /// * `from_user` - The id of the user to look the message for.
  /// * `limit` - max quantity of retrieve message.
  ///
  /// # Return
  /// * A sorted vector of message. Could be empty.
  /// * A diesel error.
  fn find(
    &self,
    conn: &DbConnection,
    from_msg: i32,
    from_user: i32,
    limit: i64,
  ) -> RepoResult<Vec<Message>>;
}

pub struct MessageRepositoryImpl;

impl MessageRepositoryImpl {
  /// Look for the last inserted message from a specific user.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `from_user` - The id of the user to look for the message.
  ///
  /// # Return
  /// * The message struct.
  /// * A diesel error.
  fn find_latest_msg(
    &self,
    conn: &DbConnection,
    from_user: i32,
  ) -> RepoResult<Message> {
    let msg = messages::table
      .filter(from.eq(from_user))
      .order(id.desc())
      .first(conn.deref())?;
    Ok(msg)
  }
}

impl MessageRepository for MessageRepositoryImpl {
  fn add(
    &self,
    conn: &DbConnection,
    new_message: NewMessage,
  ) -> RepoResult<i32> {
    diesel::insert_into(messages::table)
      .values(new_message.borrow())
      .execute(conn.deref())?;
    let msg = self.find_latest_msg(conn.borrow(), new_message.get_from())?;
    Ok(msg.get_id())
  }

  fn get(&self, conn: &DbConnection, id_msg: i32) -> RepoResult<Message> {
    let msg = messages::table.find(id_msg).get_result(conn.deref())?;
    Ok(msg)
  }

  fn find(
    &self,
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
}
