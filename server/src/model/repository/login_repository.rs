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
use diesel::prelude::*;
use std::{borrow::Borrow, ops::Deref};
use diesel::result::Error;

pub fn add(conn: &DbConnection, new_login: NewLogin) -> RepoResult<Login> {
  diesel::insert_into(logins::table)
    .values(new_login.borrow())
    .execute(conn.deref())?;

  find_by_natural_key(conn, new_login.get_username(), new_login.get_token())
}

pub fn find(
  conn: &DbConnection,
  the_username: String,
) -> RepoResult<Option<Login>> {
  match logins::table
    .filter(username.eq(the_username))
    .first::<Login>(conn.deref()) {
    Ok(login_found) => Ok(Option::from(login_found)),
    Err(err) => {
      match err {
        Error::NotFound => Ok(None),
        _ => Err(err),
      }
    }
  }
}

pub fn update(conn: &DbConnection, login: &Login) -> RepoResult<Login> {
  diesel::update(logins::table.filter(id.eq(login.get_id())))
    .set(token.eq(login.get_token()))
    .execute(conn.deref())?;

  find_by_natural_key(conn, login.get_username(), login.get_token())
}

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
