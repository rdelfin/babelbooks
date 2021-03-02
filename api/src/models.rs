use crate::schema::book;
use serde::{Deserialize, Serialize};

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
