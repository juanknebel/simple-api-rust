use std::borrow::Borrow;
use crate::model::repository::{login_repository, user_repository};
use crate::model::user::NewUser;
use crate::model::login::{Login, NewLogin};
use crate::DbConnection;
use sha2::{Digest, Sha256};

/// Creates a new user based and generates the password's hash.
///
/// # Arguments
/// * `the_username` - A string that represents the username.
/// * `password` - A string that represents the password and its going to be
/// hashed.
///
/// # Return
/// * A NewUser struct to be inserted in the database.
pub fn create_user(
    conn: DbConnection,
    username: String,
    password: String,
) -> Result<i32, String> {
    let hashed = calculate_hash(password);
    let new_user = NewUser::new(username, hashed);
    user_repository::add(new_user, conn.borrow())
}

fn calculate_hash(password: String) -> String {
    let mut hasher = Sha256::new();
    //hasher.update(password.as_ref());
    hasher.update(<String as AsRef<[u8]>>::as_ref(&password));
    format!("{:X}", hasher.finalize())
}

pub fn login(conn: DbConnection,
             username: String,
             password: String) -> Result<Login, String> {
    let hashed = calculate_hash(password);
    let search_user = NewUser::new(username, hashed);
    let user_result = user_repository::find(search_user, conn.borrow());
    match user_result {
        Ok(user) => {
            let token = String::from("ble");
            let new_login = NewLogin::new(user.get_username(), token);
            let login = login_repository::add(new_login, conn.borrow());
            Ok(login.unwrap())
        },
        Err(err) => {
            Err(err.to_string())
        }
    }
}