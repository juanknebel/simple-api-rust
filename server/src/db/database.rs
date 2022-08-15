use diesel::{
  r2d2,
  r2d2::{ConnectionManager, PooledConnection},
};
use dotenv::dotenv;
use std::env;

use rocket_contrib::databases::diesel::SqliteConnection;

type PoolType = r2d2::Pool<ConnectionManager<SqliteConnection>>;
type PooledType = PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct DbConnection {
  pool: PoolType,
}

impl DbConnection {
  pub fn new(pool: PoolType) -> Self {
    DbConnection {
      pool,
    }
  }

  pub fn get(&self) -> Result<PooledType, diesel::result::Error> {
    match self.pool.get() {
      Ok(pool) => Ok(pool),
      Err(_) => Err(diesel::result::Error::NotFound),
    }
  }
}

pub fn establish_connection() -> PoolType {
  if cfg!(test) {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create DB pool.");
    pool
  } else {
    dotenv().ok();

    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create DB pool.")
  }
}
