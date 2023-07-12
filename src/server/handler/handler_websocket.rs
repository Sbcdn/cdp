use super::super::models::{Client, Clients, WSRequest};
use crate::models::TokenInfoView;
use crate::server::filter::with_auth;
use crate::server::models::{WSMessage, WSResponse, WSResponseTypes};
use ::log::{debug, error, info};
use futures::{FutureExt, StreamExt};
use ratelimit_meter::{DirectRateLimiter, LeakyBucket};
use rweb::ws::{Message, WebSocket, Ws};
use rweb::*;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            debug!("No clients connected, skip sending data");
            continue;
        }
        debug!("{connected_client_count} connected client(s)");
        /*
        // Worker Loop task for each client
        // Lookup if messages are in que for each client and send them to the client (Redis)

                clients.lock().await.iter().for_each(|(_, client)| {
                    if let Some(sender) = &client.sender {
                        let _ = sender.send(Ok(Message::binary(
                            serde_json::to_string(&format!("Client Id: {:?}", client.client_id)).unwrap(),
                        )));
                    }
                });
            }
        */
    }
}

#[get("/")]
pub(crate) fn handle_ws_client(
    #[filter = "with_auth"] user_id: String,
    #[filter = "ws"] ws: Ws,
    #[data] rate_limiter: DirectRateLimiter<LeakyBucket>,
    #[data] clients: Clients,
) -> Result<impl rweb::Reply, http::Error> {
    debug!("ws_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(user_id, socket, clients, rate_limiter)))
}

async fn client_connection(
    user_id: String,
    ws: WebSocket,
    clients: Clients,
    _rate_limiter: DirectRateLimiter<LeakyBucket>,
) {
    debug!("establishing client connection... {ws:?}");

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {e}");
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
            //&user_id.clone(),
            msg.clone(),
            &clients.clone(),
            //&mut rate_limiter.clone(),
        )
        .await;
    }
    clients.lock().await.remove(&uuid);
    info!("{uuid} disconnected");
}

async fn client_msg(
    client_id: String,
    //_user_id: &str,
    msg: Message,
    clients: &Clients,
    // _rate_limiter: &mut DirectRateLimiter<LeakyBucket>,
) {
    debug!("received message from {client_id}: {msg:?}");
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if let Ok(m) = serde_json::from_str::<WSMessage>(message) {
        match m.request {
            WSRequest::Alive => {
                debug!("Alive");
                let locked = clients.lock().await;
                if let Some(v) = locked.get(&client_id) {
                    if let Some(sender) = &v.sender {
                        sender
                            .send(Ok(Message::text(
                                json!(WSResponse {
                                    message_id: m.message_id,
                                    response: WSResponseTypes::String("I am alive".to_string())
                                })
                                .to_string(),
                            )))
                            .unwrap();
                    }
                }
            }
            WSRequest::IsNFT(v) => {
                debug!("IsNFT");
                let dbs = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
                    db_path: std::env::var("DBSYNC_URL").unwrap(),
                }));
                let mut payload = Vec::<bool>::new();
                for f in &v {
                    if let Ok(true) = crate::dbsync::check_nft_supply(dbs.provider(), f).await {
                        payload.push(true);
                    } else {
                        payload.push(false);
                    }
                }
                send_to_client(
                    client_id,
                    clients,
                    m.message_id,
                    WSResponseTypes::VBool(payload),
                )
                .await;
            }
            WSRequest::AddressAssetHandles(v) => {
                debug!("AddressAssetHandles");
                let payload = if let Ok(o) =
                    crate::server::handler::info::get_asset_for_addresses(&v).await
                {
                    o
                } else {
                    error!("error getting asset handles for addresses: {v:?}");
                    vec![]
                };
                send_to_client(
                    client_id,
                    clients,
                    m.message_id,
                    WSResponseTypes::VAssetHandle(payload),
                )
                .await;
            }
            WSRequest::MintMetadata(v) => {
                debug!("MintMetadata");
                let mut payload = Vec::<TokenInfoView>::new();
                let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
                    db_path: std::env::var("DBSYNC_URL").unwrap(),
                }));
                for s in &v {
                    if let Ok(response) =
                        crate::provider::CardanoDataProvider::mint_metadata(&dp, &s).await
                    {
                        payload.push(response)
                    };
                }
                send_to_client(
                    client_id,
                    clients,
                    m.message_id,
                    WSResponseTypes::VTokenInfoView(payload),
                )
                .await;
            }
        }
    } else {
        let locked = clients.lock().await;
        if let Some(v) = locked.get(&client_id) {
            if let Some(sender) = &v.sender {
                sender.send(Ok(Message::text("Invalid Request"))).unwrap();
            }
        }
    }
}

async fn send_to_client(
    client_id: String,
    clients: &Clients,
    mid: String,
    payload: WSResponseTypes,
) {
    let locked = clients.lock().await;
    if let Some(v) = locked.get(&client_id) {
        if let Some(sender) = &v.sender {
            sender
                .send(Ok(Message::text(
                    json!(WSResponse {
                        message_id: mid,
                        response: payload
                    })
                    .to_string(),
                )))
                .unwrap();
        }
    }
}
