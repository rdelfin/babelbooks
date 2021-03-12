use crate::{
    models::{Book, NewBook, NewUser},
    schema::{book, user},
};
use anyhow::{anyhow, Result};
use diesel::{prelude::*, sql_types, sqlite::SqliteConnection};

// Declarations used for auto-gen IDs
no_arg_sql_function!(
    last_insert_rowid,
    sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn connect(db_url: &str) -> Result<SqliteConnection> {
    Ok(SqliteConnection::establish(&db_url)?)
}

pub fn get_all_books(connection: &SqliteConnection) -> Result<Vec<Book>> {
    use crate::schema::book::dsl::*;
    Ok(book.load::<Book>(connection)?)
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
        .ok_or_else(|| anyhow!("Failed to get book with ISBN {}", isbn_))?
        .clone())
}

pub fn add_user(connection: &SqliteConnection, username: &str, password: &str) -> Result<i32> {
    diesel::insert_into(user::table)
        .values(NewUser { username, password })
        .execute(connection)?;
    Ok(diesel::select(last_insert_rowid).get_result::<i32>(connection)?)
}
