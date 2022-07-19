use crate::schema::messages;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
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
