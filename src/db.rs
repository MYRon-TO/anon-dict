pub mod schema;
pub mod word_data;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

// use word_data::WordData;

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
}

pub struct DataBase {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DataBase {

    pub async fn new() -> Result<DataBase, DbError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // let pool = Pool::builder().build(ConnectionManager::<SqliteConnection>::new(database_url))?;
        let pool = Pool::builder()
            .max_size(10)
            .build(ConnectionManager::<SqliteConnection>::new(database_url));

        match pool {
            Ok(pool) => Ok(DataBase { pool }),
            Err(e) => Err(DbError::ConnectionError(e.to_string())),
        }
    }

    pub async fn get_clone(&self) -> Pool<ConnectionManager<SqliteConnection>>  {
        self.pool.clone()
    }

}
