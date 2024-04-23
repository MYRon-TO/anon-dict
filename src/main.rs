use anon_dict::db::DataBase;
use anon_dict::search::{Freq, fuzzy_search_from_word, search_from_word};

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = DataBase::new(database_url)
        .await
        .expect("Error creating database connection.");

    // if let Ok(results) = fuzzy_search_from_word(&db, "appel", 5, Freq::Bnc).await {
    //     for result in results {
    //         result.debug();
    //     }
    // } else {
    //     println!("No results found.");
    // }

    if let Ok(results) = search_from_word(&db, "appeals", 5, Freq::None).await {
        for result in results {
            result.debug();
        }
    } else {
        println!("No results found.");
    }
}
