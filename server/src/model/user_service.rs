use std::borrow::Borrow;

use sha2::{Digest, Sha256};

use crate::model::login::{Login, NewLogin};
use crate::model::repository::{login_repository, user_repository};
use crate::model::user::{NewUser, User};
use crate::DbConnection;

/// Creates a new user based and generates the password's hash.
///
/// # Arguments
/// * `the_username` - A string that represents the username.
/// * `password` - A string that represents the password and its going to be
/// hashed.
///
/// # Return
/// * A NewUser struct to be inserted in the database.
pub fn create_user(conn: &DbConnection, username: String, password: String) -> Result<i32, String> {
    let hashed = calculate_hash(password);
    let new_user = NewUser::new(username, hashed);
    user_repository::add(conn.borrow(), new_user)
}

fn calculate_hash(password: String) -> String {
    let mut hasher = Sha256::new();
    //hasher.update(password.as_ref());
    hasher.update(<String as AsRef<[u8]>>::as_ref(&password));
    format!("{:X}", hasher.finalize())
}

pub fn login(conn: &DbConnection, username: String, password: String) -> Result<Login, String> {
    let hashed = calculate_hash(password);
    let search_user = NewUser::new(username, hashed);
    let user_result = user_repository::find(conn.borrow(), search_user);
    match user_result {
        Ok(user) => {
            let token = create_token();
            let new_login = NewLogin::new(user.get_username(), token);
            let login = login_repository::add(conn.borrow(), new_login);
            Ok(login.unwrap())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn create_token() -> String {
    return String::from("ble")
}

pub fn total(conn: &DbConnection) -> Result<i64, String> {
    user_repository::total(conn.borrow())
}

pub fn get(conn: &DbConnection, id: i32) -> Result<User, String> {
    user_repository::get(conn.borrow(), id)
}

pub fn is_same_token(conn: &DbConnection, token: &str, user: &User) -> Result<bool, String> {
    login_repository::exist(conn.borrow(), user.get_username(), token)
}
