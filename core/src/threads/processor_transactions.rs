use std::sync::mpsc;
use crate::strucks::transaction::Transaction;
use crate::main_classes::blockchain::Blockchain;
use std::sync::{Mutex, Arc};

pub struct Processor {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub queue: mpsc::Receiver<Transaction>,
}

impl Processor {
    pub fn new(blockchain_main: Arc<Mutex<Blockchain>>, recipient: mpsc::Receiver<Transaction>) -> Processor{
        Self {
            blockchain: blockchain_main,
            queue: recipient,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.queue.recv() {
                Ok(transaction) => {
                    
                    let _function = transaction.get_function();
                    let _params = transaction.get_params();

                    let locked_blockchain = &mut self.blockchain.lock().unwrap();
                    locked_blockchain.transactions.push(transaction.clone());

                    if locked_blockchain.transactions.len() >= 100 {
                        locked_blockchain.create_block();
                    }

                } Err(_) => {
                    break;
                }
            }      
        }
    }
}