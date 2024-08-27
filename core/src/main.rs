use std::sync::mpsc;
use std::sync::{Arc, Mutex};
mod strucks;
mod cryptography;
mod server;
mod threads;
mod main_classes;

use crate::threads::server::run_server;
use crate::threads::processor_transactions::Processor;
use crate::main_classes::blockchain::Blockchain;

fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    {
        let mut locked_blockchain = blockchain.lock().unwrap();
        let _ = locked_blockchain.load();
    }

    let (sender, recipient) = mpsc::channel();

    let mut proccesor_module = Processor::new(blockchain.clone(), recipient);
    let processor_thread = std::thread::spawn(move || {
        proccesor_module.run();
    });

    let server_thread = std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run_server(sender, blockchain))
            .unwrap();
    });

    // let server_web_thread = std::thread::spawn(move || {
    //     tokio::runtime::Runtime::new()
    //         .unwrap()
    //         .block_on(server::websocket::run())
    //         .unwrap();
    // });
    
    println!("Main Modules Start... OK");
    server_thread.join().unwrap();
    processor_thread.join().unwrap();
    // server_web_thread.join().unwrap();
}