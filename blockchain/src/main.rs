// blockchain/src/main.rs
use blockchain::{Blockchain, Block, Transaction, start_server};
use wallet::Wallet;
use transaction::create_transaction;

fn main() {
    println!("Starting blockchain test...");

    let mut bc = Blockchain::new();
    let wallet = Wallet::new();
    
    println!("Initial Blockchain: {:?}", bc.chain);

    if wallet.can_transfer(10).is_ok() {
    let (from, to, amount) = create_transaction(&wallet, "acf0291c1775428198e14beca1a762a".to_string(), 10, 1.0);
    let tx = Transaction { from, to, amount };
    println!("Transaction added: From {} to {}", tx.from, tx.to);
    bc.add_block(vec![tx]);
} else {
    println!("Wallet cannot transfer 10 units.");
}

    // Rest of your code remains the same
    let new_block = Block::new(1, vec![Transaction {
        from: "acf0291c1775428198e14beca1a762a".to_string(),
        to: "e3f07eaf9e8c442f7e7e7ec8be7aec98".to_string(),
        amount: 20,
    }], bc.chain.last().unwrap().hash.clone());
    bc.chain.push(new_block);
    println!("New block added: {:?}", &bc.chain.last().unwrap());

    println!("Final Blockchain: {:?}", bc.chain);
    println!("Chain valid: {}", bc.is_chain_valid());

    if let Err(e) = start_server() {
        eprintln!("Server failed: {:?}", e);
    } else {
        println!("Server running on http://localhost:8080. Press Ctrl+C to exit.");
    }
}