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

/// Insert a user in the database
///
/// # Arguments
/// * `conn` - The database connection.
/// * `new_user` - The new user to be inserted.
///
/// # Return
/// * The id of the user.
/// * A diesel error.
pub fn add(conn: &DbConnection, new_user: NewUser) -> RepoResult<i32> {
  diesel::insert_into(users::table)
    .values(new_user.borrow())
    .execute(conn.deref())?;
  let user: User = users::table
    .filter(username.eq(new_user.get_username()))
    .get_result(conn.deref())?;
  Ok(user.get_id())
}

/// Search a user by its parameters.
///
/// # Arguments
/// * `conn` - The database connection.
/// * `the_username` - The username of the user to look for.
/// * `password` - The hashed password of the user to look for.
///
/// # Return
/// * A user struct.
/// * A diesel error.
pub fn find(
  conn: &DbConnection,
  the_username: String,
  password: String,
) -> RepoResult<User> {
  let user = users::table
    .filter(username.eq(the_username).and(hashed_password.eq(password)))
    .first(conn.deref())?;
  Ok(user)
}

/// Get the total number of users in the database.
///
/// # Arguments
/// * `conn` - The database connection.
///
/// # Return
/// * The number of users.
/// * A diesel error.
pub fn total(conn: &DbConnection) -> RepoResult<i64> {
  let size = users::table.select(count_star()).get_result(conn.deref())?;
  Ok(size)
}
