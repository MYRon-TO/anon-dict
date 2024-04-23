pub mod schema;
pub mod word_data;

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

    pub async fn new(data_base_path:String) -> Result<DataBase, DbError> {
        // let pool = Pool::builder().build(ConnectionManager::<SqliteConnection>::new(database_url))?;
        let pool = Pool::builder()
            .max_size(10)
            .build(ConnectionManager::<SqliteConnection>::new(data_base_path));

        match pool {
            Ok(pool) => Ok(DataBase { pool }),
            Err(e) => Err(DbError::ConnectionError(e.to_string())),
        }
    }

    pub async fn get_clone(&self) -> Pool<ConnectionManager<SqliteConnection>>  {
        self.pool.clone()
    }

}
