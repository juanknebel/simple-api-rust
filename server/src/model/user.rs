use crate::schema::users;

use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Clone)]
pub struct User {
  id: i32,
  username: String,
  hashed_password: String,
}

impl User {
  pub fn get_id(&self) -> i32 {
    return self.id;
  }

  pub fn get_username(&self) -> String {
    return self.username.to_string();
  }
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
  username: String,
  hashed_password: String,
}

impl NewUser {
  pub fn new(the_username: String, the_hashed_password: String) -> NewUser {
    NewUser {
      username: the_username,
      hashed_password: the_hashed_password,
    }
  }

  pub fn get_username(&self) -> String {
    return self.username.to_string();
  }

  pub fn get_password(&self) -> String {
    return self.hashed_password.to_string();
  }
}

#[cfg(test)]
pub struct Builder {
  id: Option<i32>,
  username: Option<String>,
  hashed_password: Option<String>,
}

#[cfg(test)]
impl Builder {
  pub fn new() -> Self {
    Builder {
      id: None,
      username: None,
      hashed_password: None,
    }
  }

  pub fn with_id(mut self, id: i32) -> Builder {
    self.id = Some(id);
    self
  }

  pub fn with_username(mut self, name: &str) -> Builder {
    self.username = Some(name.to_owned());
    self
  }

  pub fn with_hashed_password(mut self, hashed_password: &str) -> Builder {
    self.hashed_password = Some(hashed_password.to_owned());
    self
  }

  pub fn build(&self) -> User {
    User {
      id: *self.id.as_ref().unwrap_or(&0),
      username: String::from(self.username.as_deref().unwrap()),
      hashed_password: String::from(self.hashed_password.as_deref().unwrap()),
    }
  }
}
