use crate::schema::messages;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Clone)]
pub struct Message {
  id: i32,
  from: i32,
  to: i32,
  message: String,
}

impl Message {
  pub fn get_id(&self) -> i32 {
    return self.id;
  }

  pub fn get_message(&self) -> String {
    return self.message.to_string();
  }

  pub fn get_to(&self) -> i32 {
    return self.to;
  }
}

#[derive(Insertable, Deserialize)]
#[table_name = "messages"]
pub struct NewMessage {
  from: i32,
  to: i32,
  message: String,
}

impl NewMessage {
  pub fn new(from_user: i32, to_user: i32, the_message: String) -> NewMessage {
    NewMessage {
      from: from_user,
      to: to_user,
      message: the_message,
    }
  }

  pub fn get_from(&self) -> i32 {
    return self.from;
  }
}

#[cfg(test)]
pub struct Builder {
  id: Option<i32>,
  from: Option<i32>,
  to: Option<i32>,
  message: Option<String>,
}

#[cfg(test)]
impl Builder {
  pub fn new() -> Self {
    Builder {
      id: None,
      from: None,
      to: None,
      message: None,
    }
  }

  pub fn with_id(mut self, the_id: i32) -> Builder {
    self.id = Some(the_id);
    self
  }

  pub fn with_from(mut self, from: i32) -> Builder {
    self.from = Some(from);
    self
  }

  pub fn with_to(mut self, to: i32) -> Builder {
    self.to = Some(to);
    self
  }

  pub fn with_message(mut self, message: &str) -> Builder {
    self.message = Some(message.to_owned());
    self
  }

  pub fn build(&self) -> Message {
    Message {
      id: *self.id.as_ref().unwrap_or(&0),
      from: *self.from.as_ref().unwrap_or(&0),
      to: *self.to.as_ref().unwrap_or(&0),
      message: String::from(self.message.as_deref().unwrap()),
    }
  }
}
