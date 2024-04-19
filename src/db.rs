mod levenshtein_distance;
mod word_data;

use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions},
    Executor, Result, Row,
};
use word_data::WordData;

const DB_PATH: &str = "sqlite:///home/Myron/Git/anon_dict/assets/stardict.db";

pub struct DataBase {
    // conn: Arc<RwLock<SqlitePool>>,
    // conn: Arc<SqlitePool>,
    conn: SqlitePool,
}

impl DataBase {
    pub async fn new() -> Result<DataBase> {
        let conn = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(DB_PATH)
            .await?;

        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS "stardict" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                "word" VARCHAR(64) COLLATE NOCASE NOT NULL UNIQUE,
                "sw" VARCHAR(64) COLLATE NOCASE NOT NULL,
                "phonetic" VARCHAR(64),
                "definition" TEXT,
                "translation" TEXT,
                "pos" VARCHAR(16),
                "collins" INTEGER DEFAULT(0),
                "oxford" INTEGER DEFAULT(0),
                "tag" VARCHAR(64),
                "bnc" INTEGER DEFAULT(NULL),
                "frq" INTEGER DEFAULT(NULL),
                "exchange" TEXT,
                "detail" TEXT,
                "audio" TEXT
            );
            CREATE UNIQUE INDEX IF NOT EXISTS "stardict_1" ON stardict (id);
            CREATE UNIQUE INDEX IF NOT EXISTS "stardict_2" ON stardict (word);
            CREATE INDEX IF NOT EXISTS "stardict_3" ON stardict (sw, word collate nocase);
            CREATE INDEX IF NOT EXISTS "sd_1" ON stardict (word collate nocase);
            "#,
        )
        .await?;

        conn.execute(
            "CREATE FUNCTION distance(s1 TEXT, s2 TEXT) \
         RETURNS INTEGER \
         AS $$ \
         BEGIN \
            RETURN distance(s1, s2); \
         END; \
         $$ LANGUAGE SQL;",
        )
        .await?;

        Ok(DataBase { conn })
    }

    pub async fn conn(&self) -> SqlitePool {
        self.conn.clone()
    }

    pub async fn search(&self, sql: &str) -> Result<Vec<WordData>> {
        let mut result = Vec::new();
        let rows = sqlx::query(sql).fetch_all(&self.conn().await).await?;
        for row in rows {
            result.push(WordData {
                id: row.try_get(0)?,
                word: row.try_get(1)?,
                sw: row.try_get(2)?,
                phonetic: row.try_get(3)?,
                definition: row.try_get(4)?,
                translation: row.try_get(5)?,
                pos: row.try_get(6)?,
                collins: row.try_get(7)?,
                oxford: row.try_get(8)?,
                tag: row.try_get(9)?,
                bnc: row.try_get(10)?,
                frq: row.try_get(11)?,
                exchange: row.try_get(12)?,
                detail: row.try_get(13)?,
                audio: row.try_get(14)?,
            });
        }
        Ok(result)
    }

    pub async fn search_from_word(&self, word: &str) -> Result<Vec<WordData>> {
        let sql = format!("SELECT * FROM stardict WHERE word LIKE '%{word}%'");
        self.search(sql.as_str()).await
    }

    pub async fn fuzzy_search_from_word(&self, word: &str, limit: i32) -> Result<Vec<WordData>> {
        let sql =
            format!("SELECT * FROM stardict ORDER BY distance(word, %{word}%) LIMIT %{limit}%;");
        self.search(sql.as_str()).await
    }
}
