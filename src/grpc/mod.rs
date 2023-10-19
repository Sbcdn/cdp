use tonic::{transport::Server, Request, Response, Status};

use crate::dbsync::{epoch_change, get_tx_slot};
use crate::provider::CardanoDataProvider;
use aya_cardano::chain_follower_request_service_server::{
    ChainFollowerRequestService, ChainFollowerRequestServiceServer,
};
use aya_cardano::{
    event_response::Message, CurrentEpochResponse, EpochChangeResponse as ProtoEpochChangeResponse,
    EpochRequest, EpochRequestType, EventResponse, EventResponseType, StakeRequest,
    StakeRequestType, StateResponse, StateResponseType, ValidatorRequest, ValidatorRequestType,
};
use cardano_serialization_lib::crypto::Ed25519KeyHash;
use cardano_serialization_lib::AssetName;

use cached::{TimedCache, Cached};
use futures::lock::Mutex;

pub mod aya_cardano {
    include!("../proto/aya_cardano.rs");
    //tonic::include_proto!("aya_cardano"); // The string specified here must match the proto package name
}
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
#[derive(Debug)]
pub struct AyaCardanoRPCServer {
    short_cache: Mutex<TimedCache<Vec<u8>,Vec<u8>>>,
    long_cache: Mutex<TimedCache<Vec<u8>,Vec<u8>>>,
}
impl Default for AyaCardanoRPCServer {
    fn default() -> Self {
        AyaCardanoRPCServer {
            short_cache: Mutex::new(TimedCache::with_lifespan(60)),
            long_cache: Mutex::new(TimedCache::with_lifespan(300)),
        }
    }
}    

fn encode_prost_message<M: prost::Message>(message: &M) -> Vec<u8> {
    message.encode_to_vec()
}

#[tonic::async_trait]
impl ChainFollowerRequestService for AyaCardanoRPCServer {
    async fn epoch_change_event(
        &self,
        request: Request<EpochRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: std::env::var("DBSYNC_URL").map_err(|e| Status::internal(e.to_string()))?,
        }));

        //select * from block b where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select max(block_no) from block b where b.epoch_no = 209) order by block_no DESC;
        let rtype = request.into_inner();

        // Check cache hit
        let cache_key = encode_prost_message(&rtype);
        let mut cache = self.short_cache.lock().await;
        match cache.cache_get(&cache_key) {
            Some(cached) => {
                log::debug!("CACHE: Some found {:?}", cached);
                let output : EventResponse = prost::Message::decode(cached.as_slice()).map_err(|e| Status::internal(e.to_string()))?;
                return Ok(Response::new(output))
            },
            None => log::debug!("CACHE: None found for key: {:?}", cache_key)
        }

        let output = match rtype.r#type() {
            EpochRequestType::LatestEpochChange => {
                let current_epoch = dp.current_epoch().await.map_err(|e| Status::internal(e.to_string()))?;
                log::debug!("Current epoch: {current_epoch}");
                let resp = epoch_change(dp.provider(), Some(current_epoch))
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                EventResponse {
                    message_type: EventResponseType::EpochChangeEvent.into(),
                    message: Some(Message::EpochChange(ProtoEpochChangeResponse {
                        last_epoch: resp.last_epoch,
                        last_blockhash: resp.last_blockhash,
                        last_slot: resp.last_slot,
                        new_epoch: resp.new_epoch,
                        new_slot: resp.new_slot,
                        new_blockhash: resp.new_blockhash,
                        epoch_nonce: resp.epoch_nonce,
                        extra_entropy: if let Some(s) = resp.extra_entropy {
                            s
                        } else {
                            "".to_string()
                        },
                    })),
                }
            }
            EpochRequestType::CurrentEpoch => {
                let resp = dp.current_epoch().await.map_err(|e| Status::internal(e.to_string()))?;
                EventResponse {
                    message_type: EventResponseType::EpochChangeEvent.into(),
                    message: Some(Message::CurrentEpoch(CurrentEpochResponse {
                        current_epoch_number: resp,
                    })),
                }
            }
            EpochRequestType::SpecificEpochChange => {
                let resp = epoch_change(dp.provider(), Some(rtype.epoch))
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                EventResponse {
                    message_type: EventResponseType::EpochChangeEvent.into(),
                    message: Some(Message::EpochChange(ProtoEpochChangeResponse {
                        last_epoch: resp.last_epoch,
                        last_blockhash: resp.last_blockhash,
                        last_slot: resp.last_slot,
                        new_epoch: resp.new_epoch,
                        new_slot: resp.new_slot,
                        new_blockhash: resp.new_blockhash,
                        epoch_nonce: resp.epoch_nonce,
                        extra_entropy: if let Some(s) = resp.extra_entropy {
                            s
                        } else {
                            "".to_string()
                        },
                    })),
                }
            }
        };
        cache.cache_set(cache_key, encode_prost_message(&output));
        Ok(Response::new(output))
    }

    async fn aya_validator_status(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::ValidatorStatusState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_stake(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::ValidatorStakeState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_registration_event(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");
        let vr = request.into_inner();

        // Check cache hit
        let cache_key = encode_prost_message(&vr);
        let mut cache = self.long_cache.lock().await;
        match cache.cache_get(&cache_key) {
            Some(cached) => {
                log::debug!("CACHE: Some found {:?}", cached);
                let reply : EventResponse = prost::Message::decode(cached.as_slice()).unwrap();
                return Ok(Response::new(reply))
            },
            None => log::debug!("CACHE: None found for key: {:?}", cache_key)
        }

        let datums = find_registration_event(&vr.txhash).await;

        for d in datums {
            log::debug!("Datum: {d:?}");
            if let Ok(datum) = restore_wmreg_datum(&d.bytes) {
                let reg_datum: aya_cardano::RegistrationDatum = datum.into_inner();
                log::debug!("Registration Datum: {:?}", reg_datum);
                let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
                    db_path: std::env::var("DBSYNC_URL").map_err(|e| Status::internal(e.to_string()))?,
                }));
                let reply = EventResponse {
                    message_type: EventResponseType::ValidatorRegistrationEvent.into(),
                    message: Some(Message::ValidatorRegistration(
                        aya_cardano::ValidatorRegistrationResponse {
                            tx_hash: vr.txhash.clone(),
                            slot: get_tx_slot(dp.provider(), &vr.txhash).map_err(|e| Status::internal(e.to_string()))? as u64,
                            operator_address: reg_datum.operator_address,
                            consensus_pub_key: reg_datum.consensus_pub_key,
                            merkle_tree_root: reg_datum.merkle_tree_root,
                            cce_address: reg_datum.cce_address,
                            en_nft_name: reg_datum.en_nft_name,
                            en_owner: reg_datum.en_owner,
                            signature: reg_datum.signature,
                        },
                    )),
                };

                cache.cache_set(cache_key, encode_prost_message(&reply));
                return Ok(Response::new(reply)); // Send back our formatted greeting
            }
        }

        let reply = EventResponse {
            message_type: EventResponseType::ValidatorRegistrationEvent.into(),
            message: Some(Message::String(
                "Could not find registration transaction".to_string(),
            )),
        };

        cache.cache_set(cache_key, encode_prost_message(&reply));
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_unregistration_event(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");
        let vr = request.into_inner();

        // Check cache hit
        let cache_key = encode_prost_message(&vr);
        let mut cache = self.long_cache.lock().await;
        match cache.cache_get(&cache_key) {
            Some(cached) => {
                log::debug!("CACHE: Some found {:?}", cached);
                let reply : EventResponse = prost::Message::decode(cached.as_slice()).map_err(|e| Status::internal(e.to_string()))?;
                return Ok(Response::new(reply))
            },
            None => log::debug!("CACHE: None found for key: {:?}", cache_key)
        }

        let datums = find_registration_event(&vr.txhash).await;
        for d in datums {
            if let Ok(datum) = restore_wmreg_datum(&d.bytes) {
                let reg_datum: aya_cardano::RegistrationDatum = datum.into_inner();
                let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
                    db_path: std::env::var("DBSYNC_URL").map_err(|e| Status::internal(e.to_string()))?,
                }));
                let reply = EventResponse {
                    message_type: EventResponseType::ValidatorUnregistrationEvent.into(),
                    message: Some(Message::ValidatorRegistration(
                        aya_cardano::ValidatorRegistrationResponse {
                            tx_hash: vr.txhash.clone(),
                            slot: get_tx_slot(dp.provider(), &vr.txhash).map_err(|e| Status::internal(e.to_string()))? as u64,
                            operator_address: reg_datum.operator_address,
                            consensus_pub_key: reg_datum.consensus_pub_key,
                            merkle_tree_root: reg_datum.merkle_tree_root,
                            cce_address: reg_datum.cce_address,
                            en_nft_name: reg_datum.en_nft_name,
                            en_owner: reg_datum.en_owner,
                            signature: reg_datum.signature,
                        },
                    )),
                };

                cache.cache_set(cache_key, encode_prost_message(&reply));
                return Ok(Response::new(reply)); // Send back our formatted greeting
            }
        }
        let reply = EventResponse {
            message_type: EventResponseType::ValidatorUnregistrationEvent.into(),
            message: Some(Message::String(
                "Could not find unregistration transaction".to_string(),
            )),
        };

        cache.cache_set(cache_key, encode_prost_message(&reply));
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_stake_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = EventResponse {
            message_type: EventResponseType::DelegatorStakeEvent.into(),
            message: Some(Message::String("DelegatorStakeEvent".to_string())),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_staked(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::DelegatorStakeState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unstake_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = EventResponse {
            message_type: EventResponseType::DelegatorUnstakeEvent.into(),
            message: Some(Message::String("DelegatorUnstakeEvent".to_string())),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unbonding(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::DelegatorUnbondingState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unbonding_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        log::debug!("Got a request: {request:?}");

        let reply = EventResponse {
            message_type: EventResponseType::DelegatorUnbondingEvent.into(),
            message: Some(Message::String("DelegatorUnbondingEvent".to_string())),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

fn restore_wmreg_datum(bytes: &[u8]) -> Result<Response<aya_cardano::RegistrationDatum>, Status> {
    let datum = cardano_serialization_lib::plutus::PlutusData::from_bytes(bytes.to_vec())
        .expect("Could not deserialize PlutusData");
    log::debug!("Restored PlutusData: {:?}", datum);
    let d_str = datum
        .to_json(cardano_serialization_lib::plutus::PlutusDatumSchema::DetailedSchema)
        .expect("Could not transform PlutusData to JSON");
    log::debug!("Restored PlutusData Str: {:?}", d_str);
    let d_svalue = serde_json::from_str::<serde_json::Value>(&d_str)
        .expect("Could not transform PlutusDataJson to serde_json::Value");
    log::debug!("Deserialized Datum: \n{:?}", &d_str);
    let fields = d_svalue.get("fields").unwrap().as_array().unwrap();

    let operator_address = hex::decode(
        fields[0]
            .as_object()
            .unwrap()
            .get("bytes")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();

    let consensus_pub_key = hex::decode(
        fields[1]
            .as_object()
            .unwrap()
            .get("bytes")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();

    let merkle_tree_root = hex::decode(
        fields[2]
            .as_object()
            .unwrap()
            .get("bytes")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();

    let cce_address = hex::decode(
        fields[3]
            .as_object()
            .unwrap()
            .get("bytes")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();

    let en_nft_name = AssetName::new(
        hex::decode(
            fields[4]
                .as_object()
                .unwrap()
                .get("bytes")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    let en_owner = Ed25519KeyHash::from_bytes(
        hex::decode(
            fields[5]
                .as_object()
                .unwrap()
                .get("bytes")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    let enSignature = hex::decode(
        fields[6]
            .as_object()
            .unwrap()
            .get("bytes")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();

    Ok(Response::new(aya_cardano::RegistrationDatum {
        operator_address: std::str::from_utf8(&operator_address).unwrap().to_owned(),
        consensus_pub_key: std::str::from_utf8(&consensus_pub_key).unwrap().to_owned(),
        merkle_tree_root: std::str::from_utf8(&merkle_tree_root).unwrap().to_owned(),
        cce_address: std::str::from_utf8(&cce_address).unwrap().to_owned(),
        en_nft_name: std::str::from_utf8(&en_nft_name.name()).unwrap().to_owned(),
        en_owner: hex::encode(en_owner.to_bytes()),
        signature: general_purpose::STANDARD.encode(enSignature),
    }))
}

async fn find_registration_event(txhash: &str) -> Vec<crate::models::CDPDatum> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    dp.find_datums_for_tx(&hex::decode(txhash).unwrap())
        .await
        .unwrap()
}
