use crate::database::Book;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BookList {
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
pub struct AddBookRequest {
    pub isbn: String,
    pub title: String,
    pub author: String,
}
