use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use crate::Blockchain;
use crate::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub peg_value: String,
    pub network: String,
    // Hapus fee dari sini, ga perlu input manual
    pub signature: Option<String>,
    pub initial_balance: Option<f64>,
}

#[derive(Deserialize)]
pub struct TransactionBatchRequest {
    transactions: Vec<TransactionRequest>,
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

#[post("/transaction/single")]
pub async fn add_single_transaction(
    transaction: web::Json<TransactionRequest>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let mut blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    // Set fee otomatis 0.001
    let fee = 0.001;
    let mut tx = Transaction::new(
        transaction.from.clone(),
        transaction.to.clone(),
        transaction.amount,
        transaction.peg_value.clone(),
        transaction.network.clone(),
        fee, // Fee otomatis
    );
    tx.signature = transaction.signature.clone();
    tx.status = "pending".to_string();
    let initial_balance = transaction.initial_balance.unwrap_or(0.0);
    if blockchain.wallet.get_balance(&tx.from, &tx.network) == 0.0 && tx.from != "genesis" && initial_balance > 0.0 {
        blockchain.wallet.update_balance(&tx.from, &tx.network, initial_balance);
        println!("Initialized balance for new address {}: {:.4} {}", tx.from, initial_balance, tx.network);
    }

    println!("Processing tx: from={}, to={}, amount={}, fee={}", tx.from, tx.to, tx.amount, tx.fee);
    let sender_balance = blockchain.wallet.get_balance(&tx.from, &tx.network);
    if tx.validate().is_ok() && sender_balance >= tx.amount + tx.fee {
        let transactions = vec![tx.clone()];
        blockchain.add_block(transactions);
        if let Some(last_block) = blockchain.chain.last_mut() {
            if let Some(last_tx) = last_block.transactions.last_mut() {
                last_tx.status = "berhasil".to_string();
            }
        }
        HttpResponse::Ok().body(format!("Transaction added with txid: {}", tx.txid))
    } else {
        tx.status = "gagal".to_string();
        println!("Validation failed for {}: {:?}", tx.from, tx.validate().err().unwrap_or_else(|| "insufficient balance".into()));
        HttpResponse::BadRequest().body("Invalid transaction or insufficient balance")
    }
}

#[post("/transaction")]
pub async fn add_transaction(
    transaction: web::Json<TransactionBatchRequest>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let mut blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let mut valid_transactions = Vec::new();

    for tx_request in &transaction.transactions {
        let fee = 0.001; // Fee otomatis buat batch
        let mut tx = Transaction::new(
            tx_request.from.clone(),
            tx_request.to.clone(),
            tx_request.amount,
            tx_request.peg_value.clone(),
            tx_request.network.clone(),
            fee,
        );
        tx.signature = tx_request.signature.clone();
        tx.status = "pending".to_string();
        let initial_balance = tx_request.initial_balance.unwrap_or(0.0);
        if blockchain.wallet.get_balance(&tx.from, &tx.network) == 0.0 && tx.from != "genesis" && initial_balance > 0.0 {
            blockchain.wallet.update_balance(&tx.from, &tx.network, initial_balance);
            println!("Initialized balance for new address {}: {:.4} {}", tx.from, initial_balance, tx.network);
        }
        {
            println!("Processing tx: from={}, to={}, amount={}, fee={}", tx.from, tx.to, tx.amount, tx.fee);
        }
        let sender_balance = blockchain.wallet.get_balance(&tx.from, &tx.network);
        if tx.validate().is_ok() && sender_balance >= tx.amount + tx.fee {
            tx.status = "pending".to_string();
            valid_transactions.push(tx.clone());
        } else {
            tx.status = "gagal".to_string();
            println!("Validation failed for {}: {:?}", tx.from, tx.validate().err().unwrap_or_else(|| "insufficient balance".into()));
        }
    }

    if !valid_transactions.is_empty() {
        blockchain.add_block(valid_transactions.clone());
        HttpResponse::Ok().body(format!("{} transactions added successfully", valid_transactions.len()))
    } else {
        HttpResponse::BadRequest().body("No valid transactions to process")
    }
}

#[get("/wallet/{address}/{network}")]
pub async fn get_wallet(path: web::Path<(String, String)>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let (address, network) = path.into_inner();
    let balance = blockchain.wallet.get_balance(&address, &network);
    HttpResponse::Ok().json(serde_json::json!({"address": address, "network": network, "balance": balance}))
}

#[get("/history/{address}")]
pub async fn get_history(path: web::Path<String>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let address = path.into_inner();
    let history = blockchain.history.iter().filter(|tx| tx.from == address || tx.to == address).cloned().collect::<Vec<_>>();
    HttpResponse::Ok().json(history)
}

#[get("/transaction/{txid}")]
pub async fn get_transaction(path: web::Path<String>, data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap_or_else(|e| panic!("Lock error: {:?}", e));
    let txid = path.into_inner();
    let tx = blockchain.history.iter().find(|t| t.txid == txid);
    match tx {
        Some(transaction) => HttpResponse::Ok().json(transaction),
        None => HttpResponse::NotFound().body("Transaction not found"),
    }
}