use crate::{models::Book, structs::google};
use anyhow::Result;

pub struct GoogleBooksApi {
    api_key: String,
}

impl GoogleBooksApi {
    pub fn new() -> Result<GoogleBooksApi> {
        Ok(GoogleBooksApi {
            api_key: dotenv::var("GOOGLE_API")?,
        })
    }

    pub async fn get_book(&self, isbn: &str) -> Result<Book> {
        let url = format!(
            "https://www.googleapis.com/books/v1/volumes?q=isbn%3D{}&key={}",
            isbn, self.api_key
        );

        let response = reqwest::get(&url)
            .await?
            .json::<google::IsbnSearchResult>()
            .await?;

        let volume = &response.items[0];

        Ok(Book {
            isbn: isbn.to_string(),
            title: volume.volume_info.title.clone(),
            author: volume
                .volume_info
                .authors
                .as_ref()
                .unwrap_or(&vec![])
                .join(", "),
        })
    }
}
