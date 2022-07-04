use crate::schema::logins;

use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
pub struct Login {
    id: i32,
    username: String,
    token: String,
}

impl Login {
    pub fn get_id(self) -> i32 {
        return self.id;
    }
    pub fn get_username(self) -> String {
        return self.username;
    }
    pub fn get_token(&self) -> String {
        return self.token.to_string();
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
