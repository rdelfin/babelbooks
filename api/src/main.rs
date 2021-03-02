use crate::structs::{AddBookRequest, BookList};
use actix_web::{get, post, web, App, HttpServer, Responder};
use diesel::sqlite::SqliteConnection;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
extern crate diesel;

mod database;
mod schema;
mod structs;

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
    prod_env: PathBuf,
}

#[get("/books")]
async fn list_books(data: web::Data<AppState>) -> impl Responder {
    web::Json(BookList {
        books: database::get_all_books(&data.dbconn).unwrap(),
    })
}

#[post("/book")]
async fn add_book(req: web::Json<AddBookRequest>, data: web::Data<AppState>) -> impl Responder {
    database::add_book(&data.dbconn, &req.isbn, &req.title, &req.author).unwrap();
    web::Json(database::get_book(&data.dbconn, &req.isbn).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.ip, opt.port);
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                dbconn: database::connect(&opt.database).unwrap(),
                prod_env: dotenv::from_filename("prod.env").unwrap(),
            })
            .service(list_books)
            .service(add_book)
    })
    .bind(addr)?
    .run()
    .await
}
