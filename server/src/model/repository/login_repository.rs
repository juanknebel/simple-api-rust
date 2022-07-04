use std::borrow::Borrow;
use std::ops::Deref;
use diesel::prelude::*;
use crate::DbConnection;
use crate::model::login::{Login, NewLogin};
use crate::schema::logins;
use crate::schema::logins::{token, username};

pub fn add(new_login: NewLogin, conn: &DbConnection) -> Result<Login, String> {
    let result = diesel::insert_into(logins::table)
        .values(new_login.borrow()).execute(conn.deref());
    match result {
        Ok(_) => {
            let login_result = logins::table.filter(
                username.eq(new_login.get_username())
                    .and(token.eq(new_login.get_token())))
                .first(conn.deref());
            match login_result {
                Ok(found) => Ok(found),
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}