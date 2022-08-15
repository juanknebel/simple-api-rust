use std::borrow::Borrow;

use crate::{
  model::{
    error::ServiceResult,
    message::{Message, NewMessage},
    repository::message_repository::MessageRepository,
  },
  DbConnection,
};

pub trait MessageService: Sync + Send {
  /// Creates a new message from a user to another user. Both user must be in
  /// the system.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `from` - The user_id of the message's sender.
  /// * `to` - The user_id of the message's recipient.
  /// * `message` - The message.
  ///
  /// # Return
  /// * The id of the recently created message.
  /// * An error otherwise.
  fn create(
    &self,
    conn: &DbConnection,
    from: i32,
    to: i32,
    message: String,
  ) -> ServiceResult<i32>;

  /// Get the message from the given id.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `id` - The message_id of the message to retrieve.
  ///
  /// # Return
  /// * The user if exist.
  /// * An error instead.
  fn get(&self, conn: &DbConnection, id: i32) -> ServiceResult<Message>;

  /// Finds the messages from a specific user, since the message_id specified
  /// and with a limit. If the limit is none, then a default of 5 is used.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `from_msg` - The message_id from to retrieve.
  /// * `from_user` - The user_id of the message's sender.
  /// * `limit` - A limit of how many messages to retrieve.
  ///
  /// # Return
  /// * A vector of messages in descending order from its id. Could be empty.
  /// * An error instead.
  fn find(
    &self,
    conn: &DbConnection,
    from_msg: i32,
    from_user: i32,
    limit: Option<i64>,
  ) -> ServiceResult<Vec<Message>>;
}

pub struct MessageServiceImpl<MessageRepo> {
  message_repository: MessageRepo,
}

impl<MessageRepo> MessageServiceImpl<MessageRepo>
where
  MessageRepo: MessageRepository,
{
  pub fn new(the_message_repository: MessageRepo) -> Self {
    MessageServiceImpl {
      message_repository: the_message_repository,
    }
  }
}

impl<MessageRepo> MessageService for MessageServiceImpl<MessageRepo>
where
  MessageRepo: MessageRepository + Send + Sync,
{
  fn create(
    &self,
    conn: &DbConnection,
    from: i32,
    to: i32,
    message: String,
  ) -> ServiceResult<i32> {
    let new_message = NewMessage::new(from, to, message);
    self
      .message_repository
      .add(conn.borrow(), new_message)
      .map_err(|err| err.to_string())
  }

  fn get(&self, conn: &DbConnection, id: i32) -> ServiceResult<Message> {
    self
      .message_repository
      .get(conn.borrow(), id)
      .map_err(|err| err.to_string())
  }

  fn find(
    &self,
    conn: &DbConnection,
    from_msg: i32,
    from_user: i32,
    limit: Option<i64>,
  ) -> ServiceResult<Vec<Message>> {
    self
      .message_repository
      .find(conn.borrow(), from_msg, from_user, limit.unwrap_or(5))
      .map_err(|err| err.to_string())
  }
}
