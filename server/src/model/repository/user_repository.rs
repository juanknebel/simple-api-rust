use diesel::prelude::*;
use std::borrow::Borrow;
use std::ops::Deref;

use crate::model::user::{NewUser, User};
use crate::schema::users;
use crate::schema::users::{hashed_password, username};
use crate::DbConnection;

pub fn add(new_user: NewUser, conn: &DbConnection) -> Result<i32, String> {
    let result = diesel::insert_into(users::table)
        .values(new_user.borrow())
        .execute(conn.deref());
    match result {
        Ok(_) => {
            let user_result: Result<User, diesel::result::Error> = users::table
                .filter(username.eq(new_user.get_username()))
                .get_result(conn.deref());
            match user_result {
                Ok(user) => Ok(user.get_id()),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn find(search_user: NewUser, conn: &DbConnection) -> Result<User, String> {
    let found_result = users::table
        .filter(username.eq(search_user.get_username())
                .and(hashed_password.eq(search_user.get_password())))
        .first(conn.deref()) ;
    match found_result {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string())
    }
}
