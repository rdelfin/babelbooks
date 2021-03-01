use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn connect(db_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

#[derive(Queryable)]
pub struct Book {
    isbn: String,
    title: String,
    author: String,
}
