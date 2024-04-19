use anon_dict::db::DataBase;
use sqlx::Row;

#[tokio::test]
async fn db_test() {
    let db = DataBase::new().await.expect("Failed to connect db");
    let row = sqlx::query("SELECT word FROM stardict WHERE word LIKE 'apple'")
        .fetch_one(&db.conn().await)
        .await
        .expect("Failed to fetch row")
        .get::<String, _>("word");
    assert_eq!(row, "apple");

    let ans = db
        .search_from_word("apple")
        .await
        .expect("Failed to fetch row")
        .first()
        .unwrap()
        .word
        .clone();
    assert_eq!(ans, "apple");

}
