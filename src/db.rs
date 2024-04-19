mod levenshtein_distance;
mod word_data;

use diesel::{prelude::*};
use word_data::WordData;

const DB_PATH: &str = "sqlite:///home/Myron/Git/anon_dict/assets/stardict.db";

pub struct DataBase {
    connection: SqliteConnection,
}

// r#"
// CREATE TABLE IF NOT EXISTS "stardict" (
//     "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
//     "word" VARCHAR(64) COLLATE NOCASE NOT NULL UNIQUE,
//     "sw" VARCHAR(64) COLLATE NOCASE NOT NULL,
//     "phonetic" VARCHAR(64),
//     "definition" TEXT,
//     "translation" TEXT,
//     "pos" VARCHAR(16),
//     "collins" INTEGER DEFAULT(0),
//     "oxford" INTEGER DEFAULT(0),
//     "tag" VARCHAR(64),
//     "bnc" INTEGER DEFAULT(NULL),
//     "frq" INTEGER DEFAULT(NULL),
//     "exchange" TEXT,
//     "detail" TEXT,
//     "audio" TEXT
// );
// CREATE UNIQUE INDEX IF NOT EXISTS "stardict_1" ON stardict (id);
// CREATE UNIQUE INDEX IF NOT EXISTS "stardict_2" ON stardict (word);
// CREATE INDEX IF NOT EXISTS "stardict_3" ON stardict (sw, word collate nocase);
// CREATE INDEX IF NOT EXISTS "sd_1" ON stardict (word collate nocase);
// "#

impl DataBase {
    pub async fn new() -> Result<DataBase, ConnectionError> {
        let connection =
            SqliteConnection::establish(DB_PATH)?;
        Ok(DataBase { connection })
    }

    // pub async fn conn(&self) -> SqlitePool {
    // }

    // pub async fn search(&self, sql: &str) -> Result<Vec<WordData>> {
    // }

    // pub async fn search_from_word(&self, word: &str) -> Result<Vec<WordData>> {
    // }

    // pub async fn fuzzy_search_from_word(&self, word: &str, limit: i32) -> Result<Vec<WordData>> {
    // }
}
