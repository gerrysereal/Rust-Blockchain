use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use crate::Blockchain;
use transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub peg_value: String,
    pub network: String,
    pub fee: f64,
}

#[derive(Serialize)]
pub struct BlockResponse {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

#[get("/block/{index}")]
pub async fn get_block(path: web::Path<u64>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let block_index = path.into_inner();
    if block_index < blockchain.chain.len() as u64 {
        let block = &blockchain.chain[block_index as usize];
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
pub async fn add_transaction(
    transaction: web::Json<TransactionRequest>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let mut blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let mut tx = Transaction::new(
        transaction.sender.clone(),
        transaction.receiver.clone(),
        transaction.amount,
        transaction.peg_value.clone(),
        transaction.network.clone(),
        transaction.fee,
    );
    let sender_balance = blockchain.wallet.get_balance(&tx.sender);
    if tx.validate().is_ok() && sender_balance >= tx.amount + tx.fee {
        blockchain.add_block(vec![tx.clone()]);
        if let Some(last_block) = blockchain.chain.last_mut() {
            if let Some(last_tx) = last_block.transactions.last_mut() {
                last_tx.status = "berhasil".to_string();
            }
        }
        HttpResponse::Ok().body("Transaction added")
    } else {
        tx.status = "gagal".to_string();
        println!("Validation error: {:?}", tx.validate().err());
        if sender_balance < tx.amount + tx.fee {
            println!("Insufficient balance: {} < {}", sender_balance, tx.amount + tx.fee);
        }
        HttpResponse::BadRequest().body("Invalid transaction or insufficient balance")
    }
}

#[get("/wallet/{address}")]
pub async fn get_wallet(path: web::Path<String>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let address = path.into_inner();
    let balance = blockchain.wallet.get_balance(&address);
    HttpResponse::Ok().json(serde_json::json!({"address": address, "balance": balance}))
}

#[get("/history/{address}")]
pub async fn get_history(path: web::Path<String>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let address = path.into_inner();
    let history = blockchain.history.iter().filter(|tx| tx.sender == address || tx.receiver == address).cloned().collect::<Vec<_>>();
    HttpResponse::Ok().json(history)
}