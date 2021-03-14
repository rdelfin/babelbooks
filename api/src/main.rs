use crate::{
    gbooks::GoogleBooksApi,
    structs::{
        AddBookRequest, BookList, CreateAccountRequest, CreateAccountResponse, ListBooksRequest,
        LoginRequest, LoginResponse,
    },
};
use actix_web::{get, post, web, App, HttpServer, Responder};
use diesel::sqlite::SqliteConnection;
use structopt::StructOpt;

#[macro_use]
extern crate diesel;

mod account_manager;
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
async fn list_books(req: web::Json<ListBooksRequest>, data: web::Data<AppState>) -> impl Responder {
    let user_id = account_manager::verify(&data.dbconn, &req.session_id).unwrap();
    web::Json(BookList {
        books: database::get_all_books(&data.dbconn, user_id).unwrap(),
    })
}

#[post("/book")]
async fn add_book(req: web::Json<AddBookRequest>, data: web::Data<AppState>) -> impl Responder {
    let user_id = account_manager::verify(&data.dbconn, &req.session_id).unwrap();
    if !validation::validate_isbn(&req.isbn) {
        (Err(anyhow::anyhow!("ISBN is not valid."))).unwrap()
    }

    let book = data.books_api.get_book(&req.isbn).await.unwrap();
    database::add_book(&data.dbconn, &book.isbn, &book.title, &book.author).unwrap();
    database::link_book_user(&data.dbconn, user_id, &book.isbn).unwrap();
    web::Json(database::get_book(&data.dbconn, &req.isbn).unwrap())
}

#[post("/account/new")]
async fn new_account(
    req: web::Json<CreateAccountRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (id, session_id) =
        account_manager::create_and_login(&data.dbconn, &req.username, &req.password).unwrap();
    web::Json(CreateAccountResponse {
        username: req.username.clone(),
        id,
        session_id,
    })
}

#[post("/account/login")]
async fn login_account(req: web::Json<LoginRequest>, data: web::Data<AppState>) -> impl Responder {
    web::Json(LoginResponse {
        session_id: account_manager::login_user(&data.dbconn, &req.username, &req.password)
            .unwrap(),
    })
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
            .service(new_account)
            .service(login_account)
    })
    .bind(addr)?
    .run()
    .await
}
