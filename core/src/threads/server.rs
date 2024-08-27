use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, middleware};
use serde::{Serialize, Deserialize};
use crate::cryptography::ecdsa::ECDSA;
use crate::strucks::transaction::Transaction;
use crate::main_classes::blockchain::Blockchain;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;

#[derive(Serialize)]
struct MessageKeys {
    public_key: String,
    secret_key: String
}

#[derive(Serialize)]
struct MessageBuild {
    data: MessageKeys,
    code: i32
}

#[derive(Deserialize)]
struct ParamsGet {
    public_key: String
}

struct ServerState {
    blockchain: Arc<Mutex<Blockchain>>,
    queue: mpsc::Sender<Transaction>,
}


async fn gen_keys() -> HttpResponse {
    let (public_key, private_key) = ECDSA::generate_keys();

    let keys = MessageKeys {
        public_key: public_key.to_string(),
        secret_key: private_key.to_string()
    };

    let json = MessageBuild {
        data: keys,
        code: 0
    };

    let json_data = serde_json::to_string(&json).unwrap();
    HttpResponse::Ok().body(json_data)
}

async fn get_last_block_number(blockchain_data: web::Data<ServerState>) -> u64 {
    let blockchain_mut = blockchain_data.blockchain.lock().unwrap();
    blockchain_mut.index_last_block
}


// async fn sign_data(data: web::Json<Transaction>, request: HttpRequest, blockchain_data: web::Data<ServerState>) -> HttpResponse {
//     let transaction = data.0;
//     let signature: String;

//     let headers = request.headers();

//     if let Some(signature_h) = headers.get("SIGNATURE") {
//         signature = signature_h.to_str().unwrap_or_default().to_string();
//     } else {
//         return HttpResponse::Ok().body("Signature must be empty");
//     }

//     match transaction.get_type() {
//         Some(type_transaction) => {
//             if type_transaction == 1 {
//                 if transaction.get_sender().is_none() {
//                     return HttpResponse::Ok().body("Sender must be empty");
//                 }

//                 if transaction.get_recipient().is_none() {
//                     return HttpResponse::Ok().body("Recipient must be empty");
//                 }

//                 if transaction.get_amount().is_none() {
//                     return HttpResponse::Ok().body("Amount must be empty");
//                 }

//                 if transaction.get_timestamp().is_none() {
//                     return HttpResponse::Ok().body("Timestamp must be empty");
//                 }
//             }
//         }
//         None => {
//             return HttpResponse::Ok().body("Type must be empty");
//         }
//     }

//     let data_transaction = &transaction.build_json("signature_1".to_string());
//     let verify_ecdsa = ECDSA::verify(&data_transaction, &transaction.get_sender().unwrap_or_default(), &signature);

//     println!("{}", transaction.build_json("print".to_string()));
//     if verify_ecdsa {
//         let _ = blockchain_data.queue.send(transaction).unwrap();
//         HttpResponse::Ok().body("OK") 
//     } else {
//         HttpResponse::Ok().body("Signature error.") 
//     }
// }


async fn main_handler(mut transaction: web::Json<Transaction>, _request: HttpRequest, _blockchain_data: web::Data<ServerState>) -> HttpResponse {
    if transaction.from.is_none() {
        transaction.set_state("Fail".to_string());

    } else if transaction.function.is_none() {
        transaction.set_state("Fail".to_string())

    } else if transaction.timestamp.is_none() {
        transaction.set_state("Fail".to_string())

    } else if transaction.signature.is_none() {
        transaction.set_state("Fail".to_string())
    } 

    if transaction.state.clone().unwrap_or_default() == "Fail" {
        return HttpResponse::Ok().body("Fail"); 
    }

    HttpResponse::Ok().body("OK") 
}

// #[actix_web::main]
pub async fn run_server(sender: mpsc::Sender<Transaction>, blockchain_main: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        
            .app_data(web::Data::new(ServerState {
                blockchain: blockchain_main.clone(),
                queue: sender.clone()
            }))
            .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))
            // .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/transaction").route(web::post().to(main_handler)))
            .default_service(web::to(main_handler))
            // .default_service(web::route().guard(guard::Not(guard::Get())).to(HttpResponse::NotFound))
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}

