use crate::{
    gbooks::GoogleBooksApi,
    structs::{AddBookRequest, BookList},
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::sqlite::SqliteConnection;
use structopt::StructOpt;

#[macro_use]
extern crate diesel;

mod database;
mod gbooks;
mod models;
mod schema;
mod structs;
mod validation;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "babelbooks-api", about = "API for the Babel Books service.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long, default_value = "80")]
    port: u32,

    #[structopt(long, default_value = "0.0.0.0")]
    ip: String,

    #[structopt(long, default_value = "prod.sqlite")]
    database: String,
}

struct AppState {
    dbconn: SqliteConnection,
    books_api: GoogleBooksApi,
}

#[get("/books")]
async fn list_books(data: web::Data<AppState>) -> impl Responder {
    web::Json(BookList {
        books: database::get_all_books(&data.dbconn).unwrap(),
    })
}

#[post("/book")]
async fn add_book(req: web::Json<AddBookRequest>, data: web::Data<AppState>) -> impl Responder {
    let book = data.books_api.get_book(&req.isbn).await.unwrap();
    database::add_book(&data.dbconn, &book.isbn, &book.title, &book.author).unwrap();
    web::Json(database::get_book(&data.dbconn, &req.isbn).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.ip, opt.port);
    dotenv::from_filename("prod.env").unwrap();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                dbconn: database::connect(&opt.database).unwrap(),
                books_api: GoogleBooksApi::new().unwrap(),
            })
            .service(list_books)
            .service(add_book)
    })
    .bind(addr)?
    .run()
    .await
}
