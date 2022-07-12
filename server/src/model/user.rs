use crate::schema::users;

use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
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
