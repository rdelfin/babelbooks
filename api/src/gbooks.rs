use crate::{models::Book, structs::google};
use actix_web::client::Client;
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
        let mut client = Client::default();
        let url = format!(
            "https://www.googleapis.com/books/v1/volumes?q=isbn%3D{}&key={}",
            isbn, self.api_key
        );
        println!("URL: {}", url);
        /*let response = client
        .get(&url)
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await
        .unwrap()
        .json::<google::IsbnSearchResult>()
        .await?;*/

        let response = reqwest::get(&url)
            .await?
            .json::<google::IsbnSearchResult>()
            .await?;

        let volume = &response.items[0];

        Ok(Book {
            isbn: isbn.to_string(),
            title: volume.volumeInfo.title.clone(),
            author: volume
                .volumeInfo
                .authors
                .as_ref()
                .unwrap_or(&vec![])
                .join(", "),
        })
    }
}
