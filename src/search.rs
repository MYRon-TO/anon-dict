pub mod levenshtein_distance;

use crate::db::{word_data::WordData, DataBase};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;

use diesel::expression::functions::sql_function;
use diesel::sql_types::Text;
use levenshtein_distance::distance;

/// # Search from word
/// Search for a word in the database
pub async fn search_from_word(
    database: DataBase,
    word_to_search: &str,
) -> Result<Vec<WordData>, diesel::result::Error> {
    use crate::db::schema::stardict::dsl::*;

    // get connection
    let mut db = database.get_clone().await.get().unwrap();

    // setup levenshtein distance function
    sql_function!(fn get_distance(a: Text, b: Text) -> Integer);
    get_distance::register_impl(&mut db, |a: String, b: String| distance(a, b))?;

    stardict
        .order_by(get_distance(word_to_search, word))
        .limit(5)
        .load::<WordData>(&mut db)
}
