use super::super::models::{Client, Clients, WSCom};
use crate::server::filter::with_auth;
use futures::{FutureExt, StreamExt};
use ratelimit_meter::{DirectRateLimiter, LeakyBucket};
use rweb::ws::{Message, WebSocket, Ws};
use rweb::*;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            println!("No clients connected, skip sending data");
            continue;
        }
        println!("{} connected client(s)", connected_client_count);

        clients.lock().await.iter().for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::binary(
                    serde_json::to_string(&format!("Hello user {:?}", client.client_id)).unwrap(),
                )));
            }
        });
    }
}

#[get("/")]
pub(crate) fn handle_ws_client(
    #[filter = "with_auth"] user_id: String,
    #[filter = "ws"] ws: Ws,
    #[data] rate_limiter: DirectRateLimiter<LeakyBucket>,
    #[data] clients: Clients,
) -> Result<impl rweb::Reply, http::Error> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(user_id, socket, clients, rate_limiter)))
}

async fn client_connection(
    user_id: String,
    ws: WebSocket,
    clients: Clients,
    rate_limiter: DirectRateLimiter<LeakyBucket>,
) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().as_simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
        user_id: user_id.clone(),
    };
    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(
            uuid.clone(),
            &user_id,
            msg,
            &clients,
            &mut rate_limiter.clone(),
        )
        .await;
    }
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(
    client_id: String,
    _user_id: &str,
    msg: Message,
    clients: &Clients,
    _rate_limiter: &mut DirectRateLimiter<LeakyBucket>,
) {
    println!("received message from {}: {:?}", client_id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    match serde_json::from_str(message).unwrap() {
        //"alive" | "alive\n"
        WSCom::Alive => {
            let locked = clients.lock().await;
            match locked.get(&client_id) {
                Some(v) => {
                    if let Some(sender) = &v.sender {
                        //log::info!("sending alive");
                        let _ = sender.send(Ok(Message::text("OK")));
                    }
                }
                None => (),
            }
        }
    }
}
