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

pub fn add(conn: &DbConnection, new_user: NewUser) -> RepoResult<i32> {
  diesel::insert_into(users::table)
    .values(new_user.borrow())
    .execute(conn.deref())?;
  let user: User = users::table
    .filter(username.eq(new_user.get_username()))
    .get_result(conn.deref())?;
  Ok(user.get_id())
}

pub fn find(conn: &DbConnection, search_user: NewUser) -> RepoResult<User> {
  let user = users::table
    .filter(
      username
        .eq(search_user.get_username())
        .and(hashed_password.eq(search_user.get_password())),
    )
    .first(conn.deref())?;
  Ok(user)
}

pub fn total(conn: &DbConnection) -> RepoResult<i64> {
  let size = users::table.select(count_star()).get_result(conn.deref())?;
  Ok(size)
}

pub fn get(conn: &DbConnection, id: i32) -> RepoResult<User> {
  let user = users::table.find(id).get_result(conn.deref())?;
  Ok(user)
}
