use crate::models::Book;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BookList {
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
pub struct ListBooksRequest {
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddBookRequest {
    pub isbn: String,
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub username: String,
    pub id: i32,
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub session_id: String,
}

//======================================================
//             Google Book API structs
//======================================================

pub mod google {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct IsbnSearchResult {
        pub kind: String,
        pub totalItems: u32,
        pub items: Vec<Volume>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Volume {
        pub kind: String,
        pub id: String,
        pub etag: String,
        pub selfLink: String,
        pub volumeInfo: VolumeInfo,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct VolumeInfo {
        pub title: String,
        pub subtitle: Option<String>,
        pub authors: Option<Vec<String>>,
        pub publisher: Option<String>,
        pub publishedDate: Option<String>,
        pub description: Option<String>,
        pub industryIdentifiers: Vec<IndustryIdentifier>,
        pub pageCount: Option<u64>,
        pub printType: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct IndustryIdentifier {
        #[serde(rename = "type")]
        pub itype: String,
        pub identifier: String,
    }
}
