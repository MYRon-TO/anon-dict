use anon_dict::db::DataBase;
use anon_dict::search::search_from_word;


#[tokio::main]
async fn main() {
    let db = DataBase::new().await.expect("Error creating database connection.");

    if let Ok(results) = search_from_word(db, "did").await{
        for result in results {
            result.debug();
        }
    } else {
        println!("No results found.");
    }
}
