use crate::{
  model::{
    login::{Login, NewLogin},
    repository::error::RepoResult,
  },
  schema::{
    logins,
    logins::{token, username},
  },
  DbConnection,
};
use diesel::prelude::*;
use std::{borrow::Borrow, ops::Deref};

pub fn add(conn: &DbConnection, new_login: NewLogin) -> RepoResult<Login> {
  diesel::insert_into(logins::table)
    .values(new_login.borrow())
    .execute(conn.deref())?;

  let login = logins::table
    .filter(
      username
        .eq(new_login.get_username())
        .and(token.eq(new_login.get_token())),
    )
    .first(conn.deref())?;
  Ok(login)
}

pub fn exist(
  conn: &DbConnection,
  the_username: String,
  the_token: &str,
) -> RepoResult<bool> {
  let count: i64 = logins::table
    .filter(username.eq(the_username).and(token.eq(the_token)))
    .count()
    .get_result(conn.deref())?;
  Ok(count > 0)
}
