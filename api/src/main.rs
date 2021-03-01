use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "babelbooks-api", about = "API for the Babel Books service.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long, default_value = "80")]
    port: u32,

    #[structopt(long, default_value = "0.0.0.0")]
    ip: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.ip, opt.port);
    HttpServer::new(|| App::new().service(hello))
        .bind(addr)?
        .run()
        .await
}
