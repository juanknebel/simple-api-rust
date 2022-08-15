use sha2::{Digest, Sha256};
use std::borrow::Borrow;

use crate::{
  auth::token,
  model::{
    error::ServiceResult,
    login::{Login, NewLogin},
    repository::{
      login_repository::LoginRepository, user_repository::UserRepository,
    },
    user::{NewUser, User},
  },
  DbConnection, JwtConfig,
};

pub trait UserService: Sync + Send {
  /// Creates a new user based and generates the password's hash.
  ///
  /// # Arguments
  /// * `the_username` - A string that represents the username.
  /// * `password` - A string that represents the password and its going to be
  /// hashed.
  ///
  /// # Return
  /// * A NewUser struct to be inserted in the database.
  fn create_user(
    &self,
    conn: &DbConnection,
    username: String,
    password: String,
  ) -> ServiceResult<i32>;

  /// Creates or updates the a login for a specific user.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `jwt_config` - The jwt configuration used to generate the access token.
  /// * `username` - The username of an existing user.
  /// * `password` - The password of the given user.
  ///
  /// # Return
  /// * A login if it was successful.
  /// An error instead.
  fn login(
    &self,
    conn: &DbConnection,
    jwt_config: &JwtConfig,
    username: String,
    password: String,
  ) -> ServiceResult<Login>;

  /// Get the total number of register users.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  ///
  /// # Return
  /// * The total number of users.
  fn total(&self, conn: &DbConnection) -> ServiceResult<i64>;
}

pub struct UserServiceImpl<UserRepo, LoginRepo> {
  user_repository: UserRepo,
  login_repository: LoginRepo,
}

impl<UserRepo, LoginRepo> UserServiceImpl<UserRepo, LoginRepo>
where
  UserRepo: UserRepository,
  LoginRepo: LoginRepository,
{
  pub fn new(user_repository: UserRepo, login_repository: LoginRepo) -> Self {
    UserServiceImpl {
      user_repository,
      login_repository,
    }
  }

  /// Calculate hash for any given string using SHA256.
  ///
  /// # Arguments
  /// * `password` - The string to hashed.
  ///
  /// # Return
  /// * A string that represents the hash of the given one.
  fn calculate_hash(&self, password: String) -> String {
    let mut hasher = Sha256::new();
    // hasher.update(password.as_ref());
    hasher.update(<String as AsRef<[u8]>>::as_ref(&password));
    format!("{:X}", hasher.finalize())
  }

  /// Creates or updates a login for a valid user.
  ///
  /// # Arguments
  /// * `conn` - The database connection.
  /// * `jwt_config` - The jwt configuration used to generate the access token.
  /// * `user` - The user that wants to login.
  ///
  /// # Return
  /// * A login if it was successful.
  /// * An error instead.
  fn for_existing_user(
    &self,
    conn: &DbConnection,
    jwt_config: &JwtConfig,
    user: &User,
  ) -> ServiceResult<Login> {
    let token = self.create_token(user.get_id(), jwt_config);
    let new_login = NewLogin::new(user.get_username(), token);
    let login_result = self
      .login_repository
      .find(conn.borrow(), user.get_username());

    match login_result {
      Ok(login_found) => match login_found {
        Some(mut login) => {
          login.update(&new_login)?;
          Ok(
            self
              .login_repository
              .update(conn.borrow(), login.borrow())
              .unwrap(),
          )
        },
        None => {
          Ok(self.login_repository.add(conn.borrow(), new_login).unwrap())
        },
      },
      Err(err) => Err(err.to_string()),
    }
  }

  /// Creates a JWT token for a specific user_id
  ///
  /// # Arguments
  /// * `jwt_config` - The jwt configuration used to generate the access token.
  /// * `id` - The username of an existing user.
  ///
  /// # Return
  /// * A String that represents the JWT.
  /// An error instead.
  fn create_token(&self, id: i32, jwt_config: &JwtConfig) -> String {
    token::create_jwt(id, jwt_config).unwrap()
  }
}

impl<UserRepo, LoginRepo> UserService for UserServiceImpl<UserRepo, LoginRepo>
where
  UserRepo: UserRepository + Send + Sync,
  LoginRepo: LoginRepository + Send + Sync,
{
  fn create_user(
    &self,
    conn: &DbConnection,
    username: String,
    password: String,
  ) -> ServiceResult<i32> {
    let hashed = self.calculate_hash(password);
    let new_user = NewUser::new(username, hashed);
    self
      .user_repository
      .add(conn.borrow(), new_user)
      .map_err(|err| err.to_string())
  }

  fn login(
    &self,
    conn: &DbConnection,
    jwt_config: &JwtConfig,
    username: String,
    password: String,
  ) -> ServiceResult<Login> {
    let hashed = self.calculate_hash(password);
    let search_user = NewUser::new(username, hashed);
    let user_result = self.user_repository.find(
      conn.borrow(),
      search_user.get_username(),
      search_user.get_password(),
    );
    match user_result {
      Ok(user) => self.for_existing_user(conn, jwt_config, user.borrow()),
      Err(err) => Err(err.to_string()),
    }
  }

  fn total(&self, conn: &DbConnection) -> ServiceResult<i64> {
    self
      .user_repository
      .total(conn.borrow())
      .map_err(|err| err.to_string())
  }
}
