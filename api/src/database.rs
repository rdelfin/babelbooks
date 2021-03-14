use crate::{
    models::{Book, NewBook, NewUser, NewUserSession, User, UserSession},
    schema::{book, user, user_sessions},
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

pub fn add_user(
    connection: &SqliteConnection,
    username: &str,
    hashed_password: &str,
) -> Result<i32> {
    diesel::insert_into(user::table)
        .values(NewUser {
            username,
            password: hashed_password,
        })
        .execute(connection)?;
    Ok(diesel::select(last_insert_rowid).get_result::<i32>(connection)?)
}

pub fn get_id_for_username(connection: &SqliteConnection, username_: &str) -> Result<i32> {
    use crate::schema::user::dsl::*;
    Ok(user
        .filter(username.eq(username_))
        .limit(1)
        .load::<User>(connection)?
        .get(0)
        .ok_or_else(|| anyhow!("No user with username {}", username_))?
        .id)
}

pub fn get_hashed_password(connection: &SqliteConnection, username_: &str) -> Result<String> {
    use crate::schema::user::dsl::*;
    Ok(user
        .filter(username.eq(username_))
        .limit(1)
        .load::<User>(connection)?
        .get(0)
        .ok_or_else(|| anyhow!("No user with username {}", username_))?
        .password
        .clone())
}

pub fn add_session(connection: &SqliteConnection, session_id: &str, user_id: i32) -> Result<()> {
    diesel::insert_into(user_sessions::table)
        .values(NewUserSession {
            session_id,
            user_id,
        })
        .execute(connection)?;
    Ok(())
}

pub fn get_user_for_session(
    connection: &SqliteConnection,
    session_id_: &str,
) -> Result<Option<i32>> {
    use crate::schema::user_sessions::dsl::*;
    Ok(user_sessions
        .filter(session_id.eq(session_id_))
        .limit(1)
        .load::<UserSession>(connection)?
        .get(0)
        .map(|s| s.user_id))
}
