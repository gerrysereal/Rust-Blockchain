use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use crate::Transaction;
use crate::Blockchain;

#[derive(Serialize, Deserialize, Debug)]
struct BlockResponse {
    index: u64,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize)]
struct TransactionRequest {
    from: String,
    to: String,
    amount: u64,
}

#[get("/block/{index}")]
async fn get_block(path: web::Path<u64>) -> impl Responder {
    let bc = Blockchain::new();
    let block_index = path.into_inner();
    if block_index < bc.chain.len() as u64 {
        let block = &bc.chain[block_index as usize];
        let response = BlockResponse {
            index: block.index,
            timestamp: block.timestamp,
            previous_hash: block.previous_hash.clone(),
            hash: block.hash.clone(),
            transactions: block.transactions.clone(),
        };
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NotFound().body("Block not found")
    }
}

#[post("/transaction")]
async fn add_transaction(transaction: web::Json<TransactionRequest>) -> impl Responder {
    let mut bc = Blockchain::new();
    let tx = Transaction {
        from: transaction.from.clone(),
        to: transaction.to.clone(),
        amount: transaction.amount,
    };
    bc.add_block(vec![tx]);
    HttpResponse::Ok().body("Transaction added")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting Actix Web server on http://localhost:8080...");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_block)
            .service(add_transaction)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}