use blockchain::{Blockchain, Block};

#[tokio::test]
async fn test_blockchain_add() {
    let mut bc = Blockchain::new();
    let tx = "TestTx".to_string();
    bc.add_block(vec![tx.clone()]);
    assert_eq!(bc.chain.len(), 2);
    println!("Test block added: {:?}", bc.chain.last().unwrap());
}

#[tokio::test]
async fn test_transaction() {
    let mut bc = Blockchain::new();
    let wallet = wallet::Wallet::new();
    if wallet.can_transfer(10).is_ok() {
        let tx = transaction::create_transaction(&wallet, "Bob".to_string(), 10, 1.0);
        bc.add_block(vec![tx.clone()]);
        println!("Test transaction: {}", tx);
    }
    assert_eq!(bc.chain.len(), 2);
}