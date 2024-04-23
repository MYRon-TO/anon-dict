pub mod levenshtein_distance;

use crate::db::{word_data::WordData, DataBase};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;

use diesel::expression::functions::sql_function;
use diesel::sql_types::{Integer, Nullable, Text};
use levenshtein_distance::distance;

use dotenvy::dotenv;
use std::env;

/// # Frequency
/// Enum to choose between BNC and FRQ
/// Bnc (British National Corpus)
/// Frq (Modern Frequency)
/// None (No frequency)
pub enum Freq {
    Bnc,
    Frq,
    None,
}

/// # Search from word
/// Search for a word fuzzy _(with edit_distance)_ in the database
///
/// > ** WARNING **
/// > This function has serious performance issues
///
/// ## Parameters
/// - database: DataBase
///     - The database connection
/// - word_to_search: &str
///    - The word to search for
/// - limit: i64
///     - How many results to return , 5 is a good number
/// - freq:
///     - Whether to use BNC or FRQ for sorting
///     > ** WARNING **
///     > can not use None for sorting NOW
pub async fn fuzzy_search_from_word(
    database: &DataBase,
    word_to_search: &str,
    limit: i64,
    freq: Freq,
) -> Result<Vec<WordData>, diesel::result::Error> {
    use crate::db::schema::stardict::dsl::*;

    dotenv().ok();
    let distance_weight = env::var("DISTANCE_WEIGHT").expect("Cannot find DISTANCE_WEIGHT");
    let distance_weight: i32 = distance_weight
        .parse()
        .expect("DISTANCE_WEIGHT NOT A NUMBER");

    // get connection
    let mut db = database.get_clone().await.get().unwrap();

    // setup levenshtein distance function
    sql_function!(fn get_weight(a: Text, b: Text, freq: Nullable<Integer>) -> Integer);
    get_weight::register_impl(
        &mut db,
        move |a: String, b: String, freq: Option<i32>| match freq {
            Some(f) => {
                let freq = if f < 10000 && f > 0 { f / 1000 } else { 10 };
                distance_weight * distance(a, b) + freq
            }
            None => distance_weight * distance(a, b) + 10,
        },
    )?;

    match freq {
        Freq::Bnc => stardict
            .order_by(get_weight(word, word_to_search, bnc))
            .limit(limit)
            .load::<WordData>(&mut db),
        _ => stardict
            .order_by(get_weight(word, word_to_search, frq))
            .limit(limit)
            .load::<WordData>(&mut db),
    }
}

/// # Search from word
/// Search for a word fuzzy _(with edit_distance)_ in the database
///
/// > ** WARNING **
/// > This function has serious performance issues
///
/// ## Parameters
/// - database: DataBase
///     - The database connection
/// - word_to_search: &str
///    - The word to search for
/// - limit: i64
///     - How many results to return , 5 is a good number
/// - freq:
///     - Whether to use BNC or FRQ or Nothing for sorting
pub async fn search_from_word(
    database: &DataBase,
    word_to_search: &str,
    limit: i64,
    freq: Freq,
) -> Result<Vec<WordData>, diesel::result::Error> {
    use crate::db::schema::stardict::dsl::*;

    // get connection
    let mut db = database.get_clone().await.get().unwrap();

    let word_to_search = format!("{}%", word_to_search);

    match freq {
        Freq::Bnc => {
            if let Ok(something) = stardict
                .filter(word.like(&word_to_search))
                .filter(bnc.is_not_null())
                .order(bnc)
                .limit(limit)
                .load::<WordData>(&mut db)
            {
                if something.is_empty() {
                    stardict
                        .filter(word.like(&word_to_search))
                        .order(bnc)
                        .limit(limit)
                        .load::<WordData>(&mut db)
                } else {
                    Ok(something)
                }
            }else{
                Err(diesel::result::Error::NotFound)
            }
        }
        Freq::Frq =>{
            if let Ok(something) = stardict
                .filter(word.like(&word_to_search))
                .filter(bnc.is_not_null())
                .order(frq)
                .limit(limit)
                .load::<WordData>(&mut db)
            {
                if something.is_empty() {
                    stardict
                        .filter(word.like(&word_to_search))
                        .order(frq)
                        .limit(limit)
                        .load::<WordData>(&mut db)
                } else {
                    Ok(something)
                }
            }else{
                Err(diesel::result::Error::NotFound)
            }
        }
        Freq::None => stardict
            .filter(word.like(&word_to_search))
            .limit(limit)
            .load::<WordData>(&mut db),
    }
}
