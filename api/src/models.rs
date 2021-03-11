use crate::schema::{book, owned_books, user};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
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
    pub id: i32,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[belongs_to(User)]
#[belongs_to(Book, foreign_key = "isbn")]
pub struct OwnedBook {
    user_id: i32,
    isbn: String,
}

#[derive(Insertable)]
#[table_name = "owned_books"]
pub struct NewOwnedBook<'a> {
    user_id: i32,
    isbn: &'a str,
}
