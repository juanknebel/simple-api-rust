use std::{borrow::Borrow, ops::Deref};

use diesel::{dsl::count_star, prelude::*};

use crate::{
  model::{
    repository::error::RepoResult,
    user::{NewUser, User},
  },
  schema::{
    users,
    users::{hashed_password, username},
  },
  DbConnection,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait UserRepository {
  /// Insert a user in the database
  ///
  /// # Arguments
  /// * `new_user` - The new user to be inserted.
  ///
  /// # Return
  /// * The id of the user.
  /// * A diesel error.
  fn add(&self, new_user: NewUser) -> RepoResult<i32>;

  /// Search a user by its parameters.
  ///
  /// # Arguments
  /// * `the_username` - The username of the user to look for.
  /// * `password` - The hashed password of the user to look for.
  ///
  /// # Return
  /// * A user struct.
  /// * A diesel error.
  fn find(&self, the_username: String, password: String) -> RepoResult<User>;

  /// Get the total number of users in the database.
  ///
  /// # Arguments
  ///
  /// # Return
  /// * The number of users.
  /// * A diesel error.
  fn total(&self) -> RepoResult<i64>;
}

pub struct UserRepositoryImpl {
  db_connection: DbConnection,
}

impl UserRepositoryImpl {
  pub fn new(db_connection: DbConnection) -> Self {
    UserRepositoryImpl {
      db_connection,
    }
  }
}

impl UserRepository for UserRepositoryImpl {
  fn add(&self, new_user: NewUser) -> RepoResult<i32> {
    diesel::insert_into(users::table)
      .values(new_user.borrow())
      .execute(self.db_connection.get()?.deref())?;
    let user: User = users::table
      .filter(username.eq(new_user.get_username()))
      .get_result(self.db_connection.get()?.deref())?;
    Ok(user.get_id())
  }

  fn find(&self, the_username: String, password: String) -> RepoResult<User> {
    let user = users::table
      .filter(username.eq(the_username).and(hashed_password.eq(password)))
      .first(self.db_connection.get()?.deref())?;
    Ok(user)
  }

  fn total(&self) -> RepoResult<i64> {
    let size = users::table
      .select(count_star())
      .get_result(self.db_connection.get()?.deref())?;
    Ok(size)
  }
}
