use std::borrow::Borrow;

use crate::model::{
  error::ServiceResult,
  login::{Login, NewLogin},
  repository::{
    login_repository::LoginRepository, user_repository::UserRepository,
  },
  user::{NewUser, User},
};

use crate::model::password::PasswordHasher;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait UserService: Sync + Send {
  /// Creates a new user based and generates the password's hash.
  ///
  /// # Arguments
  /// * `username` - A string that represents the username.
  /// * `password` - A string that represents the password and its going to be
  /// hashed.
  ///
  /// # Return
  /// * A NewUser struct to be inserted in the database.
  fn create_user(
    &self,
    username: String,
    password: String,
  ) -> ServiceResult<i32>;

  /// Finds and return an existing user if the username and password matchs.
  ///
  /// # Arguments
  /// * `username` - A string that represents the username.
  /// * `password` - A string that represents the password.
  ///
  /// # Return
  /// * A User struct from the database.
  fn find_user(
    &self,
    username: String,
    password: String,
  ) -> ServiceResult<User>;

  /// Creates or updates the a login for a specific user.
  ///
  /// # Arguments
  /// * `user` - The user that is going to be logged in.
  /// * `token` - The token authentication for the user.
  ///
  /// # Return
  /// * A login if it was successful.
  /// An error instead.
  fn login(&self, user: &User, token: String) -> ServiceResult<Login>;

  /// Get the total number of register users.
  ///
  /// # Arguments
  ///
  /// # Return
  /// * The total number of users.
  fn total(&self) -> ServiceResult<i64>;
}

pub struct UserServiceImpl<UserRepo, LoginRepo, PwdHash> {
  user_repository: UserRepo,
  login_repository: LoginRepo,
  password_hasher: PwdHash,
}

impl<UserRepo, LoginRepo, PwdHash> UserServiceImpl<UserRepo, LoginRepo, PwdHash>
where
  UserRepo: UserRepository,
  LoginRepo: LoginRepository,
  PwdHash: PasswordHasher,
{
  pub fn new(
    user_repository: UserRepo,
    login_repository: LoginRepo,
    password_hasher: PwdHash,
  ) -> Self {
    UserServiceImpl {
      user_repository,
      login_repository,
      password_hasher,
    }
  }
}

impl<UserRepo, LoginRepo, PwdHash> UserService
  for UserServiceImpl<UserRepo, LoginRepo, PwdHash>
where
  UserRepo: UserRepository + Send + Sync,
  LoginRepo: LoginRepository + Send + Sync,
  PwdHash: PasswordHasher + Send + Sync,
{
  fn create_user(
    &self,
    username: String,
    password: String,
  ) -> ServiceResult<i32> {
    let hashed = self.password_hasher.hash(password.as_str());
    let new_user = NewUser::new(username, hashed);
    self
      .user_repository
      .add(new_user)
      .map_err(|err| err.to_string())
  }

  fn find_user(
    &self,
    username: String,
    password: String,
  ) -> ServiceResult<User> {
    let hashed = self.password_hasher.hash(password.as_str());
    let search_user = NewUser::new(username, hashed);
    self
      .user_repository
      .find(search_user.get_username(), search_user.get_password())
      .map_err(|err| err.to_string())
  }

  fn login(&self, user: &User, token: String) -> ServiceResult<Login> {
    let new_login = NewLogin::new(user.get_username(), token);
    let login_result = self.login_repository.find(user.get_username());

    match login_result {
      Ok(login_found) => match login_found {
        Some(mut login) => {
          login.update(&new_login)?;
          Ok(self.login_repository.update(login.borrow()).unwrap())
        },
        None => Ok(self.login_repository.add(new_login).unwrap()),
      },
      Err(err) => Err(err.to_string()),
    }
  }

  fn total(&self) -> ServiceResult<i64> {
    self.user_repository.total().map_err(|err| err.to_string())
  }
}
