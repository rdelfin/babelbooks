use crate::schema::book;
use anyhow::{anyhow, Result};
use diesel::{prelude::*, sqlite::SqliteConnection};
use serde::{Deserialize, Serialize};

pub fn connect(db_url: &str) -> Result<SqliteConnection> {
    Ok(SqliteConnection::establish(&db_url)?)
}

pub fn get_all_books(connection: &SqliteConnection) -> Result<Vec<Book>> {
    use crate::schema::book::dsl::*;
    Ok(book.load::<crate::database::Book>(connection)?)
}

pub fn add_book(
    connection: &SqliteConnection,
    isbn: &str,
    title: &str,
    author: &str,
) -> Result<()> {
    diesel::insert_into(book::table)
        .values(NewBook {
            isbn,
            title,
            author,
        })
        .execute(connection)?;
    Ok(())
}

pub fn get_book(connection: &SqliteConnection, isbn_: &str) -> Result<Book> {
    use crate::schema::book::dsl::*;
    let results = book
        .filter(isbn.eq(isbn_))
        .limit(1)
        .load::<Book>(connection)?;
    Ok(results
        .get(0)
        .ok_or(anyhow!("Failed to get book with ISBN {}", isbn_))?
        .clone())
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "book"]
pub struct NewBook<'a> {
    pub isbn: &'a str,
    pub title: &'a str,
    pub author: &'a str,
}
