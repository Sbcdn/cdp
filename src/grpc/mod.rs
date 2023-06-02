use tonic::{transport::Server, Request, Response, Status};

use aya_cardano::chain_follower_request_service_server::{
    ChainFollowerRequestService, ChainFollowerRequestServiceServer,
};
use aya_cardano::{
    EpochRequest, EpochRequestType, EventResponse, EventResponseType, StakeRequest,
    StakeRequestType, StateResponse, StateResponseType, ValidatorRequest, ValidatorRequestType,
};

use crate::dbsync::epoch_change;
use crate::provider::CardanoDataProvider;

pub mod aya_cardano {
    include!("../proto/aya_cardano.rs");
    //tonic::include_proto!("aya_cardano"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct AyaCardanoRPCServer {}

#[tonic::async_trait]
impl ChainFollowerRequestService for AyaCardanoRPCServer {
    async fn epoch_change_event(
        &self,
        request: Request<EpochRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: std::env::var("DBSYNC_URL").unwrap(),
        }));

        //select * from block b where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select max(block_no) from block b where b.epoch_no = 209) order by block_no DESC;
        let rtype = request.into_inner();
        let output = match rtype.r#type() {
            EpochRequestType::LatestEpochChange => {
                let current_epoch = dp.current_epoch().await.unwrap();
                println!("Current epoch: {current_epoch}");
                let resp = epoch_change(dp.provider(), Some(current_epoch))
                    .await
                    .unwrap();
                serde_json::json!(resp).to_string()
            }
            EpochRequestType::CurrentEpoch => {
                serde_json::json!(dp.current_epoch().await.unwrap()).to_string()
            }
            EpochRequestType::SpecificEpochChange => {
                serde_json::json!(epoch_change(dp.provider(), None).await.unwrap()).to_string()
            }
        };
        println!("Output: {output:?}");
        let reply = EventResponse {
            r#type: EventResponseType::EpochChangeEvent.into(),
            message: serde_json::json!(output).to_string(),
        };
        println!("Response: {reply:?}");
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_status(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::ValidatorStatusState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_stake(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::ValidatorStakeState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_registration_event(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = EventResponse {
            r#type: EventResponseType::ValidatorRegistrationEvent.into(),
            message: "ValidatorRegistrationEvent".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn aya_validator_unregistration_event(
        &self,
        request: Request<ValidatorRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = EventResponse {
            r#type: EventResponseType::ValidatorUnregistrationEvent.into(),
            message: "ValidatorUnregistrationEvent".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_stake_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = EventResponse {
            r#type: EventResponseType::DelegatorStakeEvent.into(),
            message: "DelegatorStakeEvent".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_staked(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::DelegatorStakeState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unstake_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = EventResponse {
            r#type: EventResponseType::DelegatorUnstakeEvent.into(),
            message: "DelegatorUnstakeEvent".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unbonding(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<StateResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = StateResponse {
            r#type: StateResponseType::DelegatorUnbondingState.into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn deligator_unbonding_event(
        &self,
        request: Request<StakeRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        println!("Got a request: {request:?}");

        let reply = EventResponse {
            r#type: EventResponseType::DelegatorUnbondingEvent.into(),
            message: "DelegatorUnbondingEvent".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
