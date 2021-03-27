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
        #[serde(rename = "totalItems")]
        pub total_items: u32,
        pub items: Vec<Volume>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Volume {
        pub kind: String,
        pub id: String,
        pub etag: String,
        #[serde(rename = "selfLink")]
        pub self_link: String,
        #[serde(rename = "volumeInfo")]
        pub volume_info: VolumeInfo,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct VolumeInfo {
        pub title: String,
        pub subtitle: Option<String>,
        pub authors: Option<Vec<String>>,
        pub publisher: Option<String>,
        #[serde(rename = "publishedDate")]
        pub published_date: Option<String>,
        pub description: Option<String>,
        #[serde(rename = "industryIdentifiers")]
        pub industry_identifiers: Vec<IndustryIdentifier>,
        #[serde(rename = "pageCount")]
        pub page_count: Option<u64>,
        #[serde(rename = "printType")]
        pub print_type: String,
        #[serde(rename = "imageLinks")]
        pub image_links: Option<ImageLinks>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct IndustryIdentifier {
        #[serde(rename = "type")]
        pub itype: String,
        pub identifier: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ImageLinks {
        pub thumbnail: String,
        #[serde(rename = "smallThumbnail")]
        pub small_thumbnail: String,
    }
}
