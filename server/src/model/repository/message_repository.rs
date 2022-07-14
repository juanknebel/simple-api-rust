use std::borrow::Borrow;
use std::ops::Deref;

use diesel::prelude::*;

use crate::model::message::{Message, NewMessage};
use crate::schema::messages;
use crate::schema::messages::{from, id};
use crate::DbConnection;

pub fn add(conn: &DbConnection, new_message: NewMessage) -> Result<i32, String> {
    let result_insert = diesel::insert_into(messages::table)
        .values(new_message.borrow())
        .execute(conn.deref());
    match result_insert {
        Ok(_) => {
            let msg_result = find_latest_msg(conn.borrow(), new_message.get_from());
            match msg_result {
                Ok(msg) => Ok(msg.get_id()),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn find_latest_msg(conn: &DbConnection, from_user: i32) -> Result<Message, String> {
    let found_result = messages::table
        .filter(from.eq(from_user))
        .order(id.desc())
        .first(conn.deref());
    match found_result {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err.to_string()),
    }
}
