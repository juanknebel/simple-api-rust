use crate::model::login::{Login, NewLogin};
use crate::schema::logins;
use crate::schema::logins::{token, username};
use crate::DbConnection;
use diesel::prelude::*;
use std::borrow::Borrow;
use std::ops::Deref;

pub fn add(conn: &DbConnection, new_login: NewLogin) -> Result<Login, String> {
    let result = diesel::insert_into(logins::table)
        .values(new_login.borrow())
        .execute(conn.deref());
    match result {
        Ok(_) => {
            let login_result = logins::table
                .filter(
                    username
                        .eq(new_login.get_username())
                        .and(token.eq(new_login.get_token())),
                )
                .first(conn.deref());
            match login_result {
                Ok(found) => Ok(found),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn exist(conn: &DbConnection, the_username: String, the_token: &str) -> Result<bool, String> {
    let result: QueryResult<i64> = logins::table
        .filter(username.eq(the_username).and(token.eq(the_token)))
        .count()
        .get_result(conn.deref());
    match result {
        Ok(count) => Ok(count > 0),
        Err(err) => Err(err.to_string()),
    }
}
