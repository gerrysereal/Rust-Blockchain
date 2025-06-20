use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Arc, Mutex};
use blockchain::{Blockchain, network::add_transaction, network::get_block};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    println!("Starting Actix Web server on http://localhost:8080...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(add_transaction)
            .service(get_block)
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Blockchain API!")
}