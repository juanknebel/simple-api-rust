use crate::{
  model::{
    login::{Login, NewLogin},
    repository::error::RepoResult,
  },
  schema::{
    logins,
    logins::{id, token, username},
  },
  DbConnection,
};
use diesel::{prelude::*, result::Error};
use std::{borrow::Borrow, ops::Deref};

pub trait LoginRepository {
  /// Insert a login in the database
  ///
  /// # Arguments
  /// * `new_login` - The new login to be inserted.
  ///
  /// # Return
  /// * The login struct.
  /// * A diesel error.
  fn add(&self, new_login: NewLogin) -> RepoResult<Login>;

  /// Look for a login for the given username.
  ///
  /// # Arguments
  /// * `username` - The username to look for.
  ///
  /// # Return
  /// * An Option for the login struct.
  /// * A diesel error.
  fn find(&self, the_username: String) -> RepoResult<Option<Login>>;

  /// Updates the login in the database
  ///
  /// # Arguments
  /// * `login` - The login to be updated.
  ///
  /// # Return
  /// * The login struct.
  /// * A diesel error.
  fn update(&self, login: &Login) -> RepoResult<Login>;
}

pub struct LoginRepositoryImpl {
  db_connection: DbConnection,
}

impl LoginRepositoryImpl {
  pub fn new(db_connection: DbConnection) -> Self {
    LoginRepositoryImpl {
      db_connection,
    }
  }
}

impl LoginRepositoryImpl {
  /// Look for a login based on its natural keys (username, token).
  ///
  /// # Arguments
  /// * `username` - The username to look for.
  /// * `token` - The token to look for.
  ///
  /// # Return
  /// * The login struct.
  /// * A diesel error.
  fn find_by_natural_key(
    &self,
    the_username: String,
    the_token: String,
  ) -> RepoResult<Login> {
    let login_updated = logins::table
      .filter(username.eq(the_username).and(token.eq(the_token)))
      .first(self.db_connection.get()?.deref())?;
    Ok(login_updated)
  }
}

impl LoginRepository for LoginRepositoryImpl {
  fn add(&self, new_login: NewLogin) -> RepoResult<Login> {
    diesel::insert_into(logins::table)
      .values(new_login.borrow())
      .execute(self.db_connection.get()?.deref())?;

    self.find_by_natural_key(new_login.get_username(), new_login.get_token())
  }

  fn find(&self, the_username: String) -> RepoResult<Option<Login>> {
    match logins::table
      .filter(username.eq(the_username))
      .first::<Login>(self.db_connection.get()?.deref())
    {
      Ok(login_found) => Ok(Option::from(login_found)),
      Err(err) => match err {
        Error::NotFound => Ok(None),
        _ => Err(err),
      },
    }
  }

  fn update(&self, login: &Login) -> RepoResult<Login> {
    diesel::update(logins::table.filter(id.eq(login.get_id())))
      .set(token.eq(login.get_token()))
      .execute(self.db_connection.get()?.deref())?;

    self.find_by_natural_key(login.get_username(), login.get_token())
  }
}
