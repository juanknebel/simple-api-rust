use crate::schema::logins;

use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Clone)]
pub struct Login {
  id: i32,
  username: String,
  token: String,
}

impl Login {
  pub fn get_id(&self) -> i32 {
    return self.id;
  }

  pub fn get_username(&self) -> String {
    return self.username.to_string();
  }

  pub fn get_token(&self) -> String {
    return self.token.to_string();
  }

  pub fn update(&mut self, login: &NewLogin) -> Result<(), String> {
    if self.username != login.get_username() {
      return Err(String::from("Username doesn't match"));
    }
    self.token = login.token.to_string();
    Ok(())
  }
}

#[derive(Insertable, Deserialize)]
#[table_name = "logins"]
pub struct NewLogin {
  username: String,
  token: String,
}

impl NewLogin {
  pub fn new(the_username: String, the_token: String) -> NewLogin {
    NewLogin {
      username: the_username,
      token: the_token,
    }
  }

  pub fn get_username(&self) -> String {
    return self.username.to_string();
  }

  pub fn get_token(&self) -> String {
    return self.token.to_string();
  }
}

#[cfg(test)]
pub struct Builder {
  id: Option<i32>,
  username: Option<String>,
  token: Option<String>,
}

#[cfg(test)]
impl Builder {
  pub fn new() -> Self {
    Builder {
      id: None,
      username: None,
      token: None,
    }
  }

  pub fn with_id(mut self, the_id: i32) -> Builder {
    self.id = Some(the_id);
    self
  }

  pub fn with_username(mut self, name: &str) -> Builder {
    self.username = Some(name.to_owned());
    self
  }

  pub fn with_token(mut self, the_token: &str) -> Builder {
    self.token = Some(the_token.to_owned());
    self
  }

  pub fn build(&self) -> Login {
    Login {
      id: *self.id.as_ref().unwrap_or(&0),
      username: String::from(self.username.as_deref().unwrap()),
      token: String::from(self.token.as_deref().unwrap()),
    }
  }
}
