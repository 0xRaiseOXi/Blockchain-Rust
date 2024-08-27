// use actix::{Actor, StreamHandler};
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use actix_web_actors::ws;
// use futures::stream::SplitSink;
// use futures::SinkExt;
// use actix::ActorContext;

// use std::collections::{HashMap, HashSet};
// use std::sync::{Arc, Mutex};

// #[derive(Default)]
// struct AppState {
//     channels: Arc<Mutex<HashMap<String, HashSet<SplitSink<ws::WebsocketContext<WebSocketActor>, ws::Message>>>>>,
// }

// struct WebSocketActor {
//     channel: String,
// }

// impl Actor for WebSocketActor {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => {
//                 // Обработка текстовых сообщений от клиента
//                 let response = format!("Channel {}: {}", self.channel, text);
//                 ctx.text(response);
//             }
//             Ok(ws::Message::Close(reason)) => {
//                 // Удаляем соединение при закрытии
//                 ctx.close(reason);
//                 let mut channels = ctx.state().channels.lock().unwrap();
//                 channels.entry(self.channel.clone()).and_modify(|set| {
//                     set.remove(&ctx);
//                 });
//             }
//             _ => {}
//         }
//     }
// }

// // Обработчик HTTP-запроса, устанавливающий соединение WebSocket
// async fn ws_handler(req: HttpRequest, stream: web::Payload, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
//     let channel: String = req.match_info().query("channel").parse().unwrap();
//     let resp = ws::start(WebSocketActor { channel }, &req, stream);

//     if let Ok(resp) = resp {
//         let mut channels = state.channels.lock().unwrap();
//         channels
//             .entry(channel.clone())
//             .or_insert_with(HashSet::new)
//             .insert(resp.0);
//     }

//     resp
// }

// pub async fn run() -> std::io::Result<()> {
//     let app_state = AppState::default();

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(app_state))
//             .service(web::resource("/ws/{channel}/").to(ws_handler))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
