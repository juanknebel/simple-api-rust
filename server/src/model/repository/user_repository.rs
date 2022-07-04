use std::borrow::Borrow;
use std::ops::Deref;

use diesel::dsl::count_star;
use diesel::prelude::*;

use crate::model::user::{NewUser, User};
use crate::schema::users;
use crate::schema::users::{hashed_password, username};
use crate::DbConnection;

pub fn add(conn: &DbConnection, new_user: NewUser) -> Result<i32, String> {
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

pub fn find(conn: &DbConnection, search_user: NewUser) -> Result<User, String> {
    let found_result = users::table
        .filter(
            username
                .eq(search_user.get_username())
                .and(hashed_password.eq(search_user.get_password())),
        )
        .first(conn.deref());
    match found_result {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string()),
    }
}

pub fn total(conn: &DbConnection) -> Result<i64, String> {
    let total_result: QueryResult<i64> = users::table.select(count_star()).get_result(conn.deref());
    match total_result {
        Ok(size) => Ok(size),
        Err(err) => Err(err.to_string()),
    }
}

pub fn get(conn: &DbConnection, id: i32) -> Result<User, String> {
    let get_result = users::table.find(id).get_result(conn.deref());
    match get_result {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string()),
    }
}
