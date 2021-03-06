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

/// Insert a login in the database
///
/// # Arguments
/// * `conn` - The database connection.
/// * `new_login` - The new login to be inserted.
///
/// # Return
/// * The login struct.
/// * A diesel error.
pub fn add(conn: &DbConnection, new_login: NewLogin) -> RepoResult<Login> {
  diesel::insert_into(logins::table)
    .values(new_login.borrow())
    .execute(conn.deref())?;

  find_by_natural_key(conn, new_login.get_username(), new_login.get_token())
}

/// Look for a login for the given username.
///
/// # Arguments
/// * `conn` - The database connection.
/// * `username` - The username to look for.
///
/// # Return
/// * An Option for the login struct.
/// * A diesel error.
pub fn find(
  conn: &DbConnection,
  the_username: String,
) -> RepoResult<Option<Login>> {
  match logins::table
    .filter(username.eq(the_username))
    .first::<Login>(conn.deref())
  {
    Ok(login_found) => Ok(Option::from(login_found)),
    Err(err) => match err {
      Error::NotFound => Ok(None),
      _ => Err(err),
    },
  }
}

/// Updates the login in the database
///
/// # Arguments
/// * `conn` - The database connection.
/// * `login` - The login to be updated.
///
/// # Return
/// * The login struct.
/// * A diesel error.
pub fn update(conn: &DbConnection, login: &Login) -> RepoResult<Login> {
  diesel::update(logins::table.filter(id.eq(login.get_id())))
    .set(token.eq(login.get_token()))
    .execute(conn.deref())?;

  find_by_natural_key(conn, login.get_username(), login.get_token())
}

/// Look for a login based on its natural keys (username, token).
///
/// # Arguments
/// * `conn` - The database connection.
/// * `username` - The username to look for.
/// * `token` - The token to look for.
///
/// # Return
/// * The login struct.
/// * A diesel error.
fn find_by_natural_key(
  conn: &DbConnection,
  the_username: String,
  the_token: String,
) -> RepoResult<Login> {
  let login_updated = logins::table
    .filter(username.eq(the_username).and(token.eq(the_token)))
    .first(conn.deref())?;
  Ok(login_updated)
}
