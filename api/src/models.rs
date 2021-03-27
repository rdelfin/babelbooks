use crate::schema::{book, owned_books, user, user_sessions};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub thumbnail: Option<String>,
    pub thumbnail_small: Option<String>,
}

#[derive(Insertable)]
#[table_name = "book"]
pub struct NewBook<'a> {
    pub isbn: &'a str,
    pub title: &'a str,
    pub author: &'a str,
    pub thumbnail: Option<&'a str>,
    pub thumbnail_small: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[belongs_to(User)]
#[belongs_to(Book, foreign_key = "isbn")]
pub struct OwnedBook {
    pub isbn: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "owned_books"]
pub struct NewOwnedBook<'a> {
    pub user_id: i32,
    pub isbn: &'a str,
}

#[derive(Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[belongs_to(User)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "user_sessions"]
pub struct NewUserSession<'a> {
    pub session_id: &'a str,
    pub user_id: i32,
}
