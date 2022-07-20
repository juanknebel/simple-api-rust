use std::borrow::Borrow;

use sha2::{Digest, Sha256};

use crate::{
  auth::token,
  model::{
    error::ServiceResult,
    login::{Login, NewLogin},
    repository::{login_repository, user_repository},
    user::{NewUser, User},
  },
  DbConnection, JwtConfig,
};

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
  conn: &DbConnection,
  username: String,
  password: String,
) -> ServiceResult<i32> {
  let hashed = calculate_hash(password);
  let new_user = NewUser::new(username, hashed);
  user_repository::add(conn.borrow(), new_user).map_err(|err| err.to_string())
}

fn calculate_hash(password: String) -> String {
  let mut hasher = Sha256::new();
  // hasher.update(password.as_ref());
  hasher.update(<String as AsRef<[u8]>>::as_ref(&password));
  format!("{:X}", hasher.finalize())
}

pub fn login(
  conn: &DbConnection,
  jwt_config: &JwtConfig,
  username: String,
  password: String,
) -> ServiceResult<Login> {
  let hashed = calculate_hash(password);
  let search_user = NewUser::new(username, hashed);
  let user_result = user_repository::find(conn.borrow(), search_user);
  match user_result {
    Ok(user) => for_existing_user(conn, jwt_config, user.borrow()),
    Err(err) => Err(err.to_string()),
  }
}

fn create_token(id: i32, jwt_config: &JwtConfig) -> String {
  token::create_jwt(id, jwt_config).unwrap()
}

pub fn total(conn: &DbConnection) -> ServiceResult<i64> {
  user_repository::total(conn.borrow()).map_err(|err| err.to_string())
}

fn for_existing_user(
  conn: &DbConnection,
  jwt_config: &JwtConfig,
  user: &User,
) -> ServiceResult<Login> {
  let token = create_token(user.get_id(), jwt_config);
  let new_login = NewLogin::new(user.get_username(), token);
  let login_result = login_repository::find(conn.borrow(), user.get_username());

  match login_result {
    Ok(login_found) => match login_found {
      Some(mut login) => {
        login.update(&new_login)?;
        Ok(login_repository::update(conn.borrow(), login.borrow()).unwrap())
      },
      None => Ok(login_repository::add(conn.borrow(), new_login).unwrap()),
    },
    Err(err) => Err(err.to_string()),
  }
}
