#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochRequest {
    #[prost(enumeration = "EpochRequestType", tag = "1")]
    pub r#type: i32,
    #[prost(int32, tag = "2")]
    pub epoch: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorRequest {
    #[prost(enumeration = "ValidatorRequestType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub txhash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeRequest {
    #[prost(enumeration = "StakeRequestType", tag = "1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventResponse {
    #[prost(enumeration = "EventResponseType", tag = "1")]
    pub message_type: i32,
    #[prost(oneof = "event_response::Message", tags = "2, 3, 4, 99")]
    pub message: ::core::option::Option<event_response::Message>,
}
/// Nested message and enum types in `EventResponse`.
pub mod event_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        EpochChange(super::EpochChangeResponse),
        #[prost(message, tag = "3")]
        CurrentEpoch(super::CurrentEpochResponse),
        #[prost(message, tag = "4")]
        ValidatorRegistration(super::ValidatorRegistrationResponse),
        #[prost(string, tag = "99")]
        String(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochChangeResponse {
    #[prost(uint64, tag = "1")]
    pub last_epoch: u64,
    #[prost(string, tag = "2")]
    pub last_blockhash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub last_slot: u64,
    #[prost(uint64, tag = "4")]
    pub new_epoch: u64,
    #[prost(uint64, tag = "5")]
    pub new_slot: u64,
    #[prost(string, tag = "6")]
    pub new_blockhash: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub epoch_nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub extra_entropy: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentEpochResponse {
    #[prost(int32, tag = "1")]
    pub current_epoch_number: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationDatum {
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub consensus_pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub merkle_tree_root: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cce_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub en_nft_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub en_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorRegistrationResponse {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub slot: u64,
    #[prost(string, tag = "3")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub consensus_pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub merkle_tree_root: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub cce_address: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub en_nft_name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub en_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateResponse {
    #[prost(enumeration = "StateResponseType", tag = "1")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EpochRequestType {
    LatestEpochChange = 0,
    SpecificEpochChange = 1,
    CurrentEpoch = 2,
}
impl EpochRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EpochRequestType::LatestEpochChange => "LatestEpochChange",
            EpochRequestType::SpecificEpochChange => "SpecificEpochChange",
            EpochRequestType::CurrentEpoch => "CurrentEpoch",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LatestEpochChange" => Some(Self::LatestEpochChange),
            "SpecificEpochChange" => Some(Self::SpecificEpochChange),
            "CurrentEpoch" => Some(Self::CurrentEpoch),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValidatorRequestType {
    AvStatus = 0,
    AvStake = 1,
    AvRegistration = 2,
    AvUnregistration = 3,
}
impl ValidatorRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValidatorRequestType::AvStatus => "AVStatus",
            ValidatorRequestType::AvStake => "AVStake",
            ValidatorRequestType::AvRegistration => "AVRegistration",
            ValidatorRequestType::AvUnregistration => "AVUnregistration",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AVStatus" => Some(Self::AvStatus),
            "AVStake" => Some(Self::AvStake),
            "AVRegistration" => Some(Self::AvRegistration),
            "AVUnregistration" => Some(Self::AvUnregistration),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StakeRequestType {
    DelegatorStakeRequest = 0,
    ValidatorStakeRequest = 1,
}
impl StakeRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StakeRequestType::DelegatorStakeRequest => "DelegatorStakeRequest",
            StakeRequestType::ValidatorStakeRequest => "ValidatorStakeRequest",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DelegatorStakeRequest" => Some(Self::DelegatorStakeRequest),
            "ValidatorStakeRequest" => Some(Self::ValidatorStakeRequest),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventResponseType {
    Error = 0,
    EpochChangeEvent = 1,
    ValidatorRegistrationEvent = 2,
    ValidatorUnregistrationEvent = 3,
    DelegatorStakeEvent = 4,
    DelegatorUnstakeEvent = 5,
    DelegatorUnbondingEvent = 6,
}
impl EventResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventResponseType::Error => "Error",
            EventResponseType::EpochChangeEvent => "EpochChangeEvent",
            EventResponseType::ValidatorRegistrationEvent => "ValidatorRegistrationEvent",
            EventResponseType::ValidatorUnregistrationEvent => {
                "ValidatorUnregistrationEvent"
            }
            EventResponseType::DelegatorStakeEvent => "DelegatorStakeEvent",
            EventResponseType::DelegatorUnstakeEvent => "DelegatorUnstakeEvent",
            EventResponseType::DelegatorUnbondingEvent => "DelegatorUnbondingEvent",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Error" => Some(Self::Error),
            "EpochChangeEvent" => Some(Self::EpochChangeEvent),
            "ValidatorRegistrationEvent" => Some(Self::ValidatorRegistrationEvent),
            "ValidatorUnregistrationEvent" => Some(Self::ValidatorUnregistrationEvent),
            "DelegatorStakeEvent" => Some(Self::DelegatorStakeEvent),
            "DelegatorUnstakeEvent" => Some(Self::DelegatorUnstakeEvent),
            "DelegatorUnbondingEvent" => Some(Self::DelegatorUnbondingEvent),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StateResponseType {
    NoStateFound = 0,
    ValidatorStatusState = 1,
    ValidatorStakeState = 2,
    DelegatorStakeState = 3,
    DelegatorUnbondingState = 4,
}
impl StateResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StateResponseType::NoStateFound => "NoStateFound",
            StateResponseType::ValidatorStatusState => "ValidatorStatusState",
            StateResponseType::ValidatorStakeState => "ValidatorStakeState",
            StateResponseType::DelegatorStakeState => "DelegatorStakeState",
            StateResponseType::DelegatorUnbondingState => "DelegatorUnbondingState",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoStateFound" => Some(Self::NoStateFound),
            "ValidatorStatusState" => Some(Self::ValidatorStatusState),
            "ValidatorStakeState" => Some(Self::ValidatorStakeState),
            "DelegatorStakeState" => Some(Self::DelegatorStakeState),
            "DelegatorUnbondingState" => Some(Self::DelegatorUnbondingState),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod chain_follower_request_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ChainFollowerRequestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChainFollowerRequestServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChainFollowerRequestServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ChainFollowerRequestServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ChainFollowerRequestServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn epoch_change_event(
            &mut self,
            request: impl tonic::IntoRequest<super::EpochRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/EpochChangeEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "EpochChangeEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aya_validator_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "AyaValidatorStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aya_validator_stake(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorStake",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "AyaValidatorStake",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aya_validator_registration_event(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorRegistrationEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "AyaValidatorRegistrationEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aya_validator_unregistration_event(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorUnregistrationEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "AyaValidatorUnregistrationEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deligator_stake_event(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/DeligatorStakeEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "DeligatorStakeEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deligator_staked(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/DeligatorStaked",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "DeligatorStaked",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deligator_unstake_event(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnstakeEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "DeligatorUnstakeEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deligator_unbonding(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnbonding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "DeligatorUnbonding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deligator_unbonding_event(
            &mut self,
            request: impl tonic::IntoRequest<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnbondingEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aya_cardano.ChainFollowerRequestService",
                        "DeligatorUnbondingEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod chain_follower_request_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChainFollowerRequestServiceServer.
    #[async_trait]
    pub trait ChainFollowerRequestService: Send + Sync + 'static {
        async fn epoch_change_event(
            &self,
            request: tonic::Request<super::EpochRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
        async fn aya_validator_status(
            &self,
            request: tonic::Request<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status>;
        async fn aya_validator_stake(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status>;
        async fn aya_validator_registration_event(
            &self,
            request: tonic::Request<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
        async fn aya_validator_unregistration_event(
            &self,
            request: tonic::Request<super::ValidatorRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
        async fn deligator_stake_event(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
        async fn deligator_staked(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status>;
        async fn deligator_unstake_event(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
        async fn deligator_unbonding(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::StateResponse>, tonic::Status>;
        async fn deligator_unbonding_event(
            &self,
            request: tonic::Request<super::StakeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ChainFollowerRequestServiceServer<T: ChainFollowerRequestService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChainFollowerRequestService> ChainFollowerRequestServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ChainFollowerRequestServiceServer<T>
    where
        T: ChainFollowerRequestService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aya_cardano.ChainFollowerRequestService/EpochChangeEvent" => {
                    #[allow(non_camel_case_types)]
                    struct EpochChangeEventSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::EpochRequest>
                    for EpochChangeEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EpochRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).epoch_change_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EpochChangeEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorStatus" => {
                    #[allow(non_camel_case_types)]
                    struct AyaValidatorStatusSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::ValidatorRequest>
                    for AyaValidatorStatusSvc<T> {
                        type Response = super::StateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aya_validator_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AyaValidatorStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorStake" => {
                    #[allow(non_camel_case_types)]
                    struct AyaValidatorStakeSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for AyaValidatorStakeSvc<T> {
                        type Response = super::StateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aya_validator_stake(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AyaValidatorStakeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorRegistrationEvent" => {
                    #[allow(non_camel_case_types)]
                    struct AyaValidatorRegistrationEventSvc<
                        T: ChainFollowerRequestService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::ValidatorRequest>
                    for AyaValidatorRegistrationEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aya_validator_registration_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AyaValidatorRegistrationEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/AyaValidatorUnregistrationEvent" => {
                    #[allow(non_camel_case_types)]
                    struct AyaValidatorUnregistrationEventSvc<
                        T: ChainFollowerRequestService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::ValidatorRequest>
                    for AyaValidatorUnregistrationEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aya_validator_unregistration_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AyaValidatorUnregistrationEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/DeligatorStakeEvent" => {
                    #[allow(non_camel_case_types)]
                    struct DeligatorStakeEventSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for DeligatorStakeEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deligator_stake_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeligatorStakeEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/DeligatorStaked" => {
                    #[allow(non_camel_case_types)]
                    struct DeligatorStakedSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for DeligatorStakedSvc<T> {
                        type Response = super::StateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deligator_staked(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeligatorStakedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnstakeEvent" => {
                    #[allow(non_camel_case_types)]
                    struct DeligatorUnstakeEventSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for DeligatorUnstakeEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deligator_unstake_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeligatorUnstakeEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnbonding" => {
                    #[allow(non_camel_case_types)]
                    struct DeligatorUnbondingSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for DeligatorUnbondingSvc<T> {
                        type Response = super::StateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deligator_unbonding(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeligatorUnbondingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aya_cardano.ChainFollowerRequestService/DeligatorUnbondingEvent" => {
                    #[allow(non_camel_case_types)]
                    struct DeligatorUnbondingEventSvc<T: ChainFollowerRequestService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ChainFollowerRequestService,
                    > tonic::server::UnaryService<super::StakeRequest>
                    for DeligatorUnbondingEventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StakeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deligator_unbonding_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeligatorUnbondingEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ChainFollowerRequestService> Clone for ChainFollowerRequestServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ChainFollowerRequestService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChainFollowerRequestService> tonic::server::NamedService
    for ChainFollowerRequestServiceServer<T> {
        const NAME: &'static str = "aya_cardano.ChainFollowerRequestService";
    }
}
