/// AddToBootstrapBlacklistRequest holds the request for AddToBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToBootstrapBlacklistRequest {
    /// IP addresses to add to bootstrap blacklist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AddToBootstrapBlacklistResponse holds the response from AddToBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToBootstrapBlacklistResponse {}
/// AddToBootstrapWhitelistRequest holds the request for AddToBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToBootstrapWhitelistRequest {
    /// IP addresses to add to bootstrap whitelist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AddToBootstrapWhitelistResponse holds the response from AddToBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToBootstrapWhitelistResponse {}
/// AddToPeersWhitelistRequest holds the request for AddToPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToPeersWhitelistRequest {
    /// IP addresses to add to peers whitelist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AddToPeersWhitelistResponse holds the response from AddToPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToPeersWhitelistResponse {}
/// AddStakingSecretKeysRequest holds the request for AddStakingSecretKeys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStakingSecretKeysRequest {
    /// Secret keys to add to wallet
    #[prost(string, repeated, tag = "1")]
    pub secret_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AddStakingSecretKeysResponse holds the response from AddStakingSecretKeys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStakingSecretKeysResponse {}
/// GetBootstrapBlacklistRequest holds the request for GetBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBootstrapBlacklistRequest {}
/// GetBootstrapBlacklistResponse holds the response from GetBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBootstrapBlacklistResponse {
    /// Bootstrap blacklisted IP addresses
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetBootstrapWhitelistRequest holds the request for GetBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBootstrapWhitelistRequest {}
/// GetBootstrapWhitelistResponse holds the response from GetBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBootstrapWhitelistResponse {
    /// Bootstrap whitelisted IP addresses
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AllowEveryoneToBootstrapRequest holds the request for AllowEveryoneToBootstrap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowEveryoneToBootstrapRequest {}
/// AllowEveryoneToBootstrapResponse holds the response from AllowEveryoneToBootstrap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowEveryoneToBootstrapResponse {}
/// GetNodeStatusRequest holds the request for GetNodeStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeStatusRequest {}
/// GetNodeStatusResponse holds the response from GetNodeStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeStatusResponse {
    /// Node status
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::model::v1::NodeStatus>,
}
/// GetPeersWhitelistRequest holds the request for GetPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeersWhitelistRequest {}
/// GetPeersWhitelistResponse holds the response from GetPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeersWhitelistResponse {
    /// Whitelisted IP addresses
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveFromBootstrapBlacklistRequest holds the request for RemoveFromBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromBootstrapBlacklistRequest {
    /// IP addresses to remove from bootstrap blacklist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveFromBootstrapBlacklistResponse holds the response from RemoveFromBootstrapBlacklist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromBootstrapBlacklistResponse {}
/// RemoveFromBootstrapWhitelistRequest holds the request for RemoveFromBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromBootstrapWhitelistRequest {
    /// IP addresses to remove from bootstrap whitelist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveFromBootstrapWhitelistResponse holds the response from RemoveFromBootstrapWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromBootstrapWhitelistResponse {}
/// RemoveFromPeersWhitelistRequest holds the request for RemoveFromPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromPeersWhitelistRequest {
    /// IP addresses to remove from peers whitelist
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveFromPeersWhitelistResponse holds the response from RemoveFromPeersWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromPeersWhitelistResponse {}
/// RemoveStakingAddressesRequest holds the request for RemoveStakingAddresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStakingAddressesRequest {
    /// Addresses to remove from staking
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveStakingAddressesResponse holds the response from RemoveStakingAddresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStakingAddressesResponse {}
/// SignMessagesRequest holds the request for SignMessages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessagesRequest {
    /// Messages to sign in bytes
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// SignMessagesResponse holds the response from SignMessages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessagesResponse {
    /// Public key that signed the message
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Signatures
    #[prost(string, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ShutdownGracefullyRequest holds the request for ShutdownGracefully
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownGracefullyRequest {}
/// ShutdownGracefullyResponse holds the response from ShutdownGracefully
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownGracefullyResponse {}
/// Generated client implementations.
pub mod private_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Massa private gRPC service
    #[derive(Debug, Clone)]
    pub struct PrivateServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PrivateServiceClient<tonic::transport::Channel> {
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
    impl<T> PrivateServiceClient<T>
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
        ) -> PrivateServiceClient<InterceptedService<T, F>>
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
            PrivateServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Add IP addresses to node bootstrap blacklist
        pub async fn add_to_bootstrap_blacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::AddToBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToBootstrapBlacklistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/AddToBootstrapBlacklist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "AddToBootstrapBlacklist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Add IP addresses to node bootstrap whitelist
        pub async fn add_to_bootstrap_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::AddToBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToBootstrapWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/AddToBootstrapWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "AddToBootstrapWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Add IP addresses to node peers whitelist. No confirmation to expect.
        /// Note: If the ip was unknown it adds it to the known peers, otherwise it updates the peer type
        pub async fn add_to_peers_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::AddToPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToPeersWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/AddToPeersWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PrivateService", "AddToPeersWhitelist"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Add staking secret keys to wallet
        pub async fn add_staking_secret_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::AddStakingSecretKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddStakingSecretKeysResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/AddStakingSecretKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "AddStakingSecretKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get node bootstrap blacklist IP addresses
        pub async fn get_bootstrap_blacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBootstrapBlacklistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/GetBootstrapBlacklist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "GetBootstrapBlacklist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get node bootstrap whitelist IP addresses
        pub async fn get_bootstrap_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBootstrapWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/GetBootstrapWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "GetBootstrapWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Allow everyone to bootstrap from the node by removing bootstrap whitelist configuration file
        pub async fn allow_everyone_to_bootstrap(
            &mut self,
            request: impl tonic::IntoRequest<super::AllowEveryoneToBootstrapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllowEveryoneToBootstrapResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/AllowEveryoneToBootstrap",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "AllowEveryoneToBootstrap",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get node status
        pub async fn get_node_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeStatusResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/GetNodeStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PrivateService", "GetNodeStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// Get node peers whitelist IP addresses
        pub async fn get_peers_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeersWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/GetPeersWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PrivateService", "GetPeersWhitelist"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove from bootstrap blacklist given IP addresses
        pub async fn remove_from_bootstrap_blacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFromBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromBootstrapBlacklistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/RemoveFromBootstrapBlacklist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "RemoveFromBootstrapBlacklist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove from bootstrap whitelist given IP addresses
        pub async fn remove_from_bootstrap_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFromBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromBootstrapWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/RemoveFromBootstrapWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "RemoveFromBootstrapWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove from peers whitelist given IP addresses
        pub async fn remove_from_peers_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFromPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromPeersWhitelistResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/RemoveFromPeersWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "RemoveFromPeersWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove addresses from staking
        pub async fn remove_staking_addresses(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveStakingAddressesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveStakingAddressesResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/RemoveStakingAddresses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PrivateService",
                        "RemoveStakingAddresses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sign messages with node's key
        pub async fn sign_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignMessagesResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/SignMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PrivateService", "SignMessages"));
            self.inner.unary(req, path, codec).await
        }
        /// Shutdown the node gracefully
        pub async fn shutdown_gracefully(
            &mut self,
            request: impl tonic::IntoRequest<super::ShutdownGracefullyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShutdownGracefullyResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PrivateService/ShutdownGracefully",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PrivateService", "ShutdownGracefully"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod private_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PrivateServiceServer.
    #[async_trait]
    pub trait PrivateService: Send + Sync + 'static {
        /// Add IP addresses to node bootstrap blacklist
        async fn add_to_bootstrap_blacklist(
            &self,
            request: tonic::Request<super::AddToBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToBootstrapBlacklistResponse>,
            tonic::Status,
        >;
        /// Add IP addresses to node bootstrap whitelist
        async fn add_to_bootstrap_whitelist(
            &self,
            request: tonic::Request<super::AddToBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToBootstrapWhitelistResponse>,
            tonic::Status,
        >;
        /// Add IP addresses to node peers whitelist. No confirmation to expect.
        /// Note: If the ip was unknown it adds it to the known peers, otherwise it updates the peer type
        async fn add_to_peers_whitelist(
            &self,
            request: tonic::Request<super::AddToPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddToPeersWhitelistResponse>,
            tonic::Status,
        >;
        /// Add staking secret keys to wallet
        async fn add_staking_secret_keys(
            &self,
            request: tonic::Request<super::AddStakingSecretKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddStakingSecretKeysResponse>,
            tonic::Status,
        >;
        /// Get node bootstrap blacklist IP addresses
        async fn get_bootstrap_blacklist(
            &self,
            request: tonic::Request<super::GetBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBootstrapBlacklistResponse>,
            tonic::Status,
        >;
        /// Get node bootstrap whitelist IP addresses
        async fn get_bootstrap_whitelist(
            &self,
            request: tonic::Request<super::GetBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBootstrapWhitelistResponse>,
            tonic::Status,
        >;
        /// Allow everyone to bootstrap from the node by removing bootstrap whitelist configuration file
        async fn allow_everyone_to_bootstrap(
            &self,
            request: tonic::Request<super::AllowEveryoneToBootstrapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllowEveryoneToBootstrapResponse>,
            tonic::Status,
        >;
        /// Get node status
        async fn get_node_status(
            &self,
            request: tonic::Request<super::GetNodeStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeStatusResponse>,
            tonic::Status,
        >;
        /// Get node peers whitelist IP addresses
        async fn get_peers_whitelist(
            &self,
            request: tonic::Request<super::GetPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeersWhitelistResponse>,
            tonic::Status,
        >;
        /// Remove from bootstrap blacklist given IP addresses
        async fn remove_from_bootstrap_blacklist(
            &self,
            request: tonic::Request<super::RemoveFromBootstrapBlacklistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromBootstrapBlacklistResponse>,
            tonic::Status,
        >;
        /// Remove from bootstrap whitelist given IP addresses
        async fn remove_from_bootstrap_whitelist(
            &self,
            request: tonic::Request<super::RemoveFromBootstrapWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromBootstrapWhitelistResponse>,
            tonic::Status,
        >;
        /// Remove from peers whitelist given IP addresses
        async fn remove_from_peers_whitelist(
            &self,
            request: tonic::Request<super::RemoveFromPeersWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFromPeersWhitelistResponse>,
            tonic::Status,
        >;
        /// Remove addresses from staking
        async fn remove_staking_addresses(
            &self,
            request: tonic::Request<super::RemoveStakingAddressesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveStakingAddressesResponse>,
            tonic::Status,
        >;
        /// Sign messages with node's key
        async fn sign_messages(
            &self,
            request: tonic::Request<super::SignMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignMessagesResponse>,
            tonic::Status,
        >;
        /// Shutdown the node gracefully
        async fn shutdown_gracefully(
            &self,
            request: tonic::Request<super::ShutdownGracefullyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShutdownGracefullyResponse>,
            tonic::Status,
        >;
    }
    /// Massa private gRPC service
    #[derive(Debug)]
    pub struct PrivateServiceServer<T: PrivateService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PrivateService> PrivateServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PrivateServiceServer<T>
    where
        T: PrivateService,
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
                "/massa.api.v1.PrivateService/AddToBootstrapBlacklist" => {
                    #[allow(non_camel_case_types)]
                    struct AddToBootstrapBlacklistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::AddToBootstrapBlacklistRequest>
                    for AddToBootstrapBlacklistSvc<T> {
                        type Response = super::AddToBootstrapBlacklistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddToBootstrapBlacklistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_to_bootstrap_blacklist(request).await
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
                        let method = AddToBootstrapBlacklistSvc(inner);
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
                "/massa.api.v1.PrivateService/AddToBootstrapWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct AddToBootstrapWhitelistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::AddToBootstrapWhitelistRequest>
                    for AddToBootstrapWhitelistSvc<T> {
                        type Response = super::AddToBootstrapWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddToBootstrapWhitelistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_to_bootstrap_whitelist(request).await
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
                        let method = AddToBootstrapWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/AddToPeersWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct AddToPeersWhitelistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::AddToPeersWhitelistRequest>
                    for AddToPeersWhitelistSvc<T> {
                        type Response = super::AddToPeersWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddToPeersWhitelistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_to_peers_whitelist(request).await
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
                        let method = AddToPeersWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/AddStakingSecretKeys" => {
                    #[allow(non_camel_case_types)]
                    struct AddStakingSecretKeysSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::AddStakingSecretKeysRequest>
                    for AddStakingSecretKeysSvc<T> {
                        type Response = super::AddStakingSecretKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddStakingSecretKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_staking_secret_keys(request).await
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
                        let method = AddStakingSecretKeysSvc(inner);
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
                "/massa.api.v1.PrivateService/GetBootstrapBlacklist" => {
                    #[allow(non_camel_case_types)]
                    struct GetBootstrapBlacklistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::GetBootstrapBlacklistRequest>
                    for GetBootstrapBlacklistSvc<T> {
                        type Response = super::GetBootstrapBlacklistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBootstrapBlacklistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_bootstrap_blacklist(request).await
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
                        let method = GetBootstrapBlacklistSvc(inner);
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
                "/massa.api.v1.PrivateService/GetBootstrapWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct GetBootstrapWhitelistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::GetBootstrapWhitelistRequest>
                    for GetBootstrapWhitelistSvc<T> {
                        type Response = super::GetBootstrapWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBootstrapWhitelistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_bootstrap_whitelist(request).await
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
                        let method = GetBootstrapWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/AllowEveryoneToBootstrap" => {
                    #[allow(non_camel_case_types)]
                    struct AllowEveryoneToBootstrapSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::AllowEveryoneToBootstrapRequest>
                    for AllowEveryoneToBootstrapSvc<T> {
                        type Response = super::AllowEveryoneToBootstrapResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AllowEveryoneToBootstrapRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).allow_everyone_to_bootstrap(request).await
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
                        let method = AllowEveryoneToBootstrapSvc(inner);
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
                "/massa.api.v1.PrivateService/GetNodeStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeStatusSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::GetNodeStatusRequest>
                    for GetNodeStatusSvc<T> {
                        type Response = super::GetNodeStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNodeStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_node_status(request).await
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
                        let method = GetNodeStatusSvc(inner);
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
                "/massa.api.v1.PrivateService/GetPeersWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct GetPeersWhitelistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::GetPeersWhitelistRequest>
                    for GetPeersWhitelistSvc<T> {
                        type Response = super::GetPeersWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPeersWhitelistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_peers_whitelist(request).await
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
                        let method = GetPeersWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/RemoveFromBootstrapBlacklist" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFromBootstrapBlacklistSvc<T: PrivateService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<
                        super::RemoveFromBootstrapBlacklistRequest,
                    > for RemoveFromBootstrapBlacklistSvc<T> {
                        type Response = super::RemoveFromBootstrapBlacklistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveFromBootstrapBlacklistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_from_bootstrap_blacklist(request).await
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
                        let method = RemoveFromBootstrapBlacklistSvc(inner);
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
                "/massa.api.v1.PrivateService/RemoveFromBootstrapWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFromBootstrapWhitelistSvc<T: PrivateService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<
                        super::RemoveFromBootstrapWhitelistRequest,
                    > for RemoveFromBootstrapWhitelistSvc<T> {
                        type Response = super::RemoveFromBootstrapWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveFromBootstrapWhitelistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_from_bootstrap_whitelist(request).await
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
                        let method = RemoveFromBootstrapWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/RemoveFromPeersWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFromPeersWhitelistSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::RemoveFromPeersWhitelistRequest>
                    for RemoveFromPeersWhitelistSvc<T> {
                        type Response = super::RemoveFromPeersWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveFromPeersWhitelistRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_from_peers_whitelist(request).await
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
                        let method = RemoveFromPeersWhitelistSvc(inner);
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
                "/massa.api.v1.PrivateService/RemoveStakingAddresses" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveStakingAddressesSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::RemoveStakingAddressesRequest>
                    for RemoveStakingAddressesSvc<T> {
                        type Response = super::RemoveStakingAddressesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveStakingAddressesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_staking_addresses(request).await
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
                        let method = RemoveStakingAddressesSvc(inner);
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
                "/massa.api.v1.PrivateService/SignMessages" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessagesSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::SignMessagesRequest>
                    for SignMessagesSvc<T> {
                        type Response = super::SignMessagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sign_messages(request).await
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
                        let method = SignMessagesSvc(inner);
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
                "/massa.api.v1.PrivateService/ShutdownGracefully" => {
                    #[allow(non_camel_case_types)]
                    struct ShutdownGracefullySvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::ShutdownGracefullyRequest>
                    for ShutdownGracefullySvc<T> {
                        type Response = super::ShutdownGracefullyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShutdownGracefullyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).shutdown_gracefully(request).await
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
                        let method = ShutdownGracefullySvc(inner);
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
    impl<T: PrivateService> Clone for PrivateServiceServer<T> {
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
    impl<T: PrivateService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PrivateService> tonic::server::NamedService for PrivateServiceServer<T> {
        const NAME: &'static str = "massa.api.v1.PrivateService";
    }
}
/// GetBlocksRequest holds request for GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    /// Returns all the blocks that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<GetBlocksFilter>,
}
/// GetBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksFilter {
    /// Filter
    #[prost(oneof = "get_blocks_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<get_blocks_filter::Filter>,
}
/// Nested message and enum types in `GetBlocksFilter`.
pub mod get_blocks_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of this creator addresses
        #[prost(message, tag = "1")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of this block ids
        #[prost(message, tag = "2")]
        BlockIds(super::super::super::model::v1::BlockIds),
        /// One of this slot ranges (inclusive)
        #[prost(message, tag = "3")]
        SlotRange(super::super::super::model::v1::SlotRange),
    }
}
/// GetBlocksResponse holds response from GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    /// Wrapped blocks
    #[prost(message, repeated, tag = "1")]
    pub wrapped_blocks: ::prost::alloc::vec::Vec<super::super::model::v1::BlockWrapper>,
}
/// GetDatastoreEntriesRequest holds request from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesRequest {
    /// Returns all the datastore entries that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<GetDatastoreEntryFilter>,
}
/// DatastoreEntryFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntryFilter {
    /// Filter
    #[prost(oneof = "get_datastore_entry_filter::Filter", tags = "1")]
    pub filter: ::core::option::Option<get_datastore_entry_filter::Filter>,
}
/// Nested message and enum types in `GetDatastoreEntryFilter`.
pub mod get_datastore_entry_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of this (address-key) pairs
        #[prost(message, tag = "1")]
        AddressKey(super::super::super::model::v1::AddressKeyEntry),
    }
}
/// GetDatastoreEntriesResponse holds response from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesResponse {
    /// Datastore entries
    #[prost(message, repeated, tag = "1")]
    pub datastore_entries: ::prost::alloc::vec::Vec<
        super::super::model::v1::DatastoreEntry,
    >,
}
/// GetMipStatusRequest holds request for GetMipStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMipStatusRequest {}
/// GetMipStatusResponse holds response from GetMipStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMipStatusResponse {
    /// (MipInfo - status id) entries
    #[prost(message, repeated, tag = "1")]
    pub mipstatus_entries: ::prost::alloc::vec::Vec<
        super::super::model::v1::MipStatusEntry,
    >,
}
/// GetNextBlockBestParentsRequest holds request for GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsRequest {}
/// GetNextBlockBestParentsResponse holds response from GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsResponse {
    /// Next block best parents
    #[prost(message, repeated, tag = "1")]
    pub block_parents: ::prost::alloc::vec::Vec<super::super::model::v1::BlockParent>,
}
/// GetOperationsRequest holds request for GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsRequest {
    /// Returns all the operations that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<GetOperationsFilter>,
}
/// GetOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsFilter {
    /// Filter
    #[prost(oneof = "get_operations_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<get_operations_filter::Filter>,
}
/// Nested message and enum types in `GetOperationsFilter`.
pub mod get_operations_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of the operation ids
        #[prost(message, tag = "1")]
        OperationIds(super::super::super::model::v1::OperationIds),
        /// One of the operation types
        #[prost(message, tag = "2")]
        OperationTypes(super::super::super::model::v1::OpTypes),
    }
}
/// GetOperationsResponse holds response from GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsResponse {
    /// Wrapped operations
    #[prost(message, repeated, tag = "1")]
    pub wrapped_operations: ::prost::alloc::vec::Vec<
        super::super::model::v1::OperationWrapper,
    >,
}
/// GetScExecutionEventsRequest holds request for GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsRequest {
    /// Returns all the sc execution events that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<ScExecutionEventsFilter>,
}
/// ScExecutionEvents Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventsFilter {
    /// Slot range (Optional)
    #[prost(message, optional, tag = "1")]
    pub slot_range: ::core::option::Option<super::super::model::v1::SlotRange>,
    /// Caller address (Optional)
    #[prost(message, optional, tag = "2")]
    pub caller_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Emitter address (Optional)
    #[prost(message, optional, tag = "3")]
    pub emitter_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Original operation id (Optional)
    #[prost(message, optional, tag = "4")]
    pub original_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status (Optional)
    #[prost(
        enumeration = "super::super::model::v1::ScExecutionEventStatus",
        repeated,
        tag = "5"
    )]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// GetScExecutionEventsResponse holds response from GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsResponse {
    /// ScExecutionEvents
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::super::model::v1::ScExecutionEvent>,
}
/// GetStatusRequest holds request from GetStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusRequest {}
/// GetStatusResponse holds request from GetStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusResponse {
    /// Status
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::model::v1::PublicStatus>,
}
/// GetSelectorDrawsRequest holds request from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsRequest {
    /// Returns all the selector draws that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<SelectorDrawsFilter>,
}
/// SelectorDraws Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDrawsFilter {
    /// Returns all the selector draws that verify all the filters
    #[prost(oneof = "selector_draws_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<selector_draws_filter::Filter>,
}
/// Nested message and enum types in `SelectorDrawsFilter`.
pub mod selector_draws_filter {
    /// Returns all the selector draws that verify all the filters
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of the Addresses
        #[prost(message, tag = "1")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of the Slot range
        #[prost(message, tag = "2")]
        SlotRange(super::super::super::model::v1::SlotRange),
    }
}
/// GetSelectorDrawsResponse holds response from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsResponse {
    /// Selector draws
    #[prost(message, repeated, tag = "1")]
    pub draws: ::prost::alloc::vec::Vec<super::super::model::v1::SlotDraw>,
}
/// GetStakersRequest holds request from GetStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStakersRequest {
    /// Returns all the stakers that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<StakersFilter>,
}
/// Stakers Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakersFilter {
    /// Returns all the stakers that verify all the filters
    #[prost(oneof = "stakers_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<stakers_filter::Filter>,
}
/// Nested message and enum types in `StakersFilter`.
pub mod stakers_filter {
    /// Returns all the stakers that verify all the filters
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Minimum rolls (Optional)
        #[prost(message, tag = "1")]
        MinRolls(u64),
        /// Maximum rolls (Optional)
        #[prost(message, tag = "2")]
        MaxRolls(u64),
        /// Limit (Optional)
        #[prost(message, tag = "3")]
        Limit(u64),
    }
}
/// GetStakersResponse holds response from GetStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStakersResponse {
    /// Stakers
    #[prost(message, repeated, tag = "1")]
    pub stakers: ::prost::alloc::vec::Vec<super::super::model::v1::StakerEntry>,
}
/// GetTransactionsThroughputRequest holds request for GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputRequest {}
/// GetTransactionsThroughputResponse holds response from GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputResponse {
    /// Transactions throughput
    #[prost(uint32, tag = "1")]
    pub throughput: u32,
}
/// Request to atomically execute a batch of execution state queries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStateRequest {
    /// List of execution query request items
    #[prost(message, optional, tag = "1")]
    pub requests: ::core::option::Option<ExecutionQueryRequestItem>,
}
/// Query state query item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryRequestItem {
    #[prost(
        oneof = "execution_query_request_item::RequestItem",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20"
    )]
    pub request_item: ::core::option::Option<execution_query_request_item::RequestItem>,
}
/// Nested message and enum types in `ExecutionQueryRequestItem`.
pub mod execution_query_request_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestItem {
        /// Checks if address exists (candidate)
        #[prost(message, tag = "1")]
        AddressExistsCandidate(super::AddressExistsCandidate),
        /// Checks if address exists (final)
        #[prost(message, tag = "2")]
        AddressExistsFinal(super::AddressExistsFinal),
        /// Gets the balance (candidate) of an address
        #[prost(message, tag = "3")]
        AddressBalanceCandidate(super::AddressBalanceCandidate),
        /// Gets the balance (final) of an address
        #[prost(message, tag = "4")]
        AddressBalanceFinal(super::AddressBalanceFinal),
        /// Gets the bytecode (candidate) of an address
        #[prost(message, tag = "5")]
        AddressBytecodeCandidate(super::AddressBytecodeCandidate),
        /// Gets the bytecode (final) of an address
        #[prost(message, tag = "6")]
        AddressBytecodeFinal(super::AddressBytecodeFinal),
        /// Gets the datastore keys (candidate) of an address
        #[prost(message, tag = "7")]
        AddressDatastoreKeysCandidate(super::AddressDatastoreKeysCandidate),
        /// Gets the datastore keys (final) of an address
        #[prost(message, tag = "8")]
        AddressDatastoreKeysFinal(super::AddressDatastoreKeysFinal),
        /// Gets a datastore value (candidate) for an address
        #[prost(message, tag = "9")]
        AddressDatastoreValueCandidate(super::AddressDatastoreValueCandidate),
        /// Gets a datastore value (final) for an address
        #[prost(message, tag = "10")]
        AddressDatastoreValueFinal(super::AddressDatastoreValueFinal),
        /// Gets the execution status (candidate) for an operation
        #[prost(message, tag = "11")]
        OpExecutionStatusCandidate(super::OpExecutionStatusCandidate),
        /// Gets the execution status (final) for an operation
        #[prost(message, tag = "12")]
        OpExecutionStatusFinal(super::OpExecutionStatusFinal),
        /// Gets the execution status (candidate) for a denunciation
        #[prost(message, tag = "13")]
        DenunciationExecutionStatusCandidate(
            super::DenunciationExecutionStatusCandidate,
        ),
        /// Gets the execution status (final) for a denunciation
        #[prost(message, tag = "14")]
        DenunciationExecutionStatusFinal(super::DenunciationExecutionStatusFinal),
        /// Gets the roll count (candidate) of an address
        #[prost(message, tag = "15")]
        AddressRollsCandidate(super::AddressRollsCandidate),
        /// Gets the roll count (final) of an address
        #[prost(message, tag = "16")]
        AddressRollsFinal(super::AddressRollsFinal),
        /// Gets the deferred credits (candidate) of an address
        #[prost(message, tag = "17")]
        AddressDeferredCreditsCandidate(super::AddressDeferredCreditsCandidate),
        /// Gets the deferred credits (final) of an address
        #[prost(message, tag = "18")]
        AddressDeferredCreditsFinal(super::AddressDeferredCreditsFinal),
        /// Gets all information for a given cycle
        #[prost(message, tag = "19")]
        CycleInfos(super::CycleInfos),
        /// Gets filtered events
        #[prost(message, tag = "20")]
        Events(super::Events),
    }
}
/// Request to check if address exists (candidate)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressExistsCandidate {
    /// Address to check
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to check if address exists (final)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressExistsFinal {
    /// Address to check
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the balance (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalanceCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the balance (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalanceFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the bytecode (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytecodeCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the bytecode (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytecodeFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the datastore keys (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreKeysCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Prefix for the keys
    #[prost(bytes = "vec", tag = "2")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get the datastore keys (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreKeysFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Prefix for the keys
    #[prost(bytes = "vec", tag = "2")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get a datastore value (candidate) for an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreValueCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Key for the value
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get a datastore value (final) for an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreValueFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Key for the value
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get the execution status (candidate) for an operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpExecutionStatusCandidate {
    /// Operation ID to query
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
}
/// Request to get the execution status (final) for an operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpExecutionStatusFinal {
    /// Operation ID to query
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
}
/// Request to get the execution status (candidate) for a denunciation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationExecutionStatusCandidate {
    /// Denunciation index to query
    #[prost(message, optional, tag = "1")]
    pub denunciation_index: ::core::option::Option<
        super::super::model::v1::DenunciationIndex,
    >,
}
/// Request to get the execution status (final) for a denunciation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationExecutionStatusFinal {
    /// Denunciation index to query
    #[prost(message, optional, tag = "1")]
    pub denunciation_index: ::core::option::Option<
        super::super::model::v1::DenunciationIndex,
    >,
}
/// Request to get the roll count (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressRollsCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the roll count (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressRollsFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the deferred credits (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDeferredCreditsCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the deferred credits (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDeferredCreditsFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get all information for a given cycle
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CycleInfos {
    /// Cycle to query
    #[prost(uint64, tag = "1")]
    pub cycle: u64,
    /// Addresses to restrict the query (if None, info for all addresses will be returned)
    #[prost(string, repeated, tag = "2")]
    pub restrict_to_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to get filtered events
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    /// Event filter to apply
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<ScExecutionEventsFilter>,
}
/// Response to atomically execute a batch of execution state queries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStateResponse {
    /// Final cursor position
    #[prost(message, optional, tag = "1")]
    pub final_cursor: ::core::option::Option<super::super::model::v1::Slot>,
    /// Candidate cursor position
    #[prost(message, optional, tag = "2")]
    pub candidate_cursor: ::core::option::Option<super::super::model::v1::Slot>,
    /// List of execution query response items
    #[prost(message, repeated, tag = "3")]
    pub responses: ::prost::alloc::vec::Vec<ExecutionQueryResponse>,
}
/// Execution state query response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryResponse {
    /// Result or error
    #[prost(oneof = "execution_query_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<execution_query_response::Response>,
}
/// Nested message and enum types in `ExecutionQueryResponse`.
pub mod execution_query_response {
    /// Result or error
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Execution query response item
        #[prost(message, tag = "1")]
        Result(super::ExecutionQueryResponseItem),
        /// Massa error
        #[prost(message, tag = "2")]
        Error(super::super::super::model::v1::Error),
    }
}
/// Execution state query response item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryResponseItem {
    #[prost(
        oneof = "execution_query_response_item::ResponseItem",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub response_item: ::core::option::Option<
        execution_query_response_item::ResponseItem,
    >,
}
/// Nested message and enum types in `ExecutionQueryResponseItem`.
pub mod execution_query_response_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseItem {
        /// Boolean value
        #[prost(bool, tag = "1")]
        Boolean(bool),
        /// Roll counts value
        #[prost(uint64, tag = "2")]
        RollCount(u64),
        /// Amount value
        #[prost(message, tag = "3")]
        Amount(super::super::super::model::v1::NativeAmount),
        /// Bytes value
        #[prost(bytes, tag = "4")]
        Bytes(::prost::alloc::vec::Vec<u8>),
        /// Vector of bytes value
        #[prost(message, tag = "5")]
        VecBytes(super::super::super::model::v1::ArrayOfBytesWrapper),
        /// Deferred credits value
        #[prost(uint64, tag = "6")]
        DeferredCredits(u64),
        /// Execution status value
        #[prost(enumeration = "super::ExecutionQueryExecutionStatus", tag = "7")]
        ExecutionStatus(i32),
        /// Cycle infos value
        #[prost(message, tag = "8")]
        CycleInfos(super::ExecutionQueryCycleInfos),
        /// Events
        #[prost(message, tag = "9")]
        Events(super::ScOutputEventsWrapper),
    }
}
/// Cycle information for execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryCycleInfos {
    /// Cycle number
    #[prost(uint64, tag = "1")]
    pub cycle: u64,
    /// Whether the cycle is final
    #[prost(bool, tag = "2")]
    pub is_final: bool,
    /// Infos for each PoS-participating address among the ones that were asked
    #[prost(message, repeated, tag = "3")]
    pub staker_infos: ::prost::alloc::vec::Vec<ExecutionQueryStakerInfoEntry>,
}
/// Staker information for a given cycle
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoEntry {
    /// Address of the staker
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Staker info
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<ExecutionQueryStakerInfo>,
}
/// Staker information for execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfo {
    /// Active roll count
    #[prost(uint64, tag = "1")]
    pub active_rolls: u64,
    /// Production stats
    #[prost(message, repeated, tag = "2")]
    pub production_stats: ::prost::alloc::vec::Vec<
        ExecutionQueryStakerInfoProductionStatsEntry,
    >,
}
/// ExecutionQueryStakerInfoProductionStats entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoProductionStatsEntry {
    /// Address of the staker
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Production failure
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<ExecutionQueryStakerInfoProductionStats>,
}
/// Production statistics for staker info in execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoProductionStats {
    /// Production successes
    #[prost(uint64, tag = "1")]
    pub block_success_count: u64,
    /// Production failures
    #[prost(uint64, tag = "2")]
    pub block_failure_count: u64,
}
/// SCOutputEvents wrapper
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScOutputEventsWrapper {
    /// Events
    #[prost(message, repeated, tag = "1")]
    pub event: ::prost::alloc::vec::Vec<super::super::model::v1::ScExecutionEvent>,
}
/// NewBlocksRequest holds request for NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksRequest {}
/// NewBlocksResponse holds response from NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksResponse {
    /// Signed block
    #[prost(message, optional, tag = "1")]
    pub signed_block: ::core::option::Option<super::super::model::v1::SignedBlock>,
}
/// NewBlocksHeadersRequest holds request for NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersRequest {}
/// NewBlocksHeadersResponse holds response from NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersResponse {
    /// Signed block header
    #[prost(message, optional, tag = "1")]
    pub signed_block_header: ::core::option::Option<
        super::super::model::v1::SignedBlockHeader,
    >,
}
/// NewEndorsementsRequest holds request for NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsRequest {}
/// NewEndorsementsResponse holds response from NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsResponse {
    /// Signed endorsement
    #[prost(message, optional, tag = "1")]
    pub signed_endorsement: ::core::option::Option<
        super::super::model::v1::SignedEndorsement,
    >,
}
/// NewFilledBlocksRequest holds request for NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksRequest {}
/// NewFilledBlocksResponse holds response from NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksResponse {
    /// Block with operations content found in the node.
    #[prost(message, optional, tag = "1")]
    pub filled_block: ::core::option::Option<super::super::model::v1::FilledBlock>,
}
/// NewOperationsRequest holds request for NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsRequest {
    /// Returns all the operations that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<NewOperationsFilter>,
}
/// NewOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsFilter {
    /// Filter
    #[prost(oneof = "new_operations_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<new_operations_filter::Filter>,
}
/// Nested message and enum types in `NewOperationsFilter`.
pub mod new_operations_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of the operation ids
        #[prost(message, tag = "1")]
        OperationIds(super::super::super::model::v1::OperationIds),
        /// One of the operation types
        #[prost(message, tag = "2")]
        OperationTypes(super::super::super::model::v1::OpTypes),
    }
}
/// NewOperationsResponse holds response from NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsResponse {
    /// Signed operation
    #[prost(message, optional, tag = "1")]
    pub signed_operation: ::core::option::Option<
        super::super::model::v1::SignedOperation,
    >,
}
/// NewSlotExecutionOutputsRequest holds request for NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsRequest {
    /// Returns all the slot execution outputs that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<NewSlotExecutionOutputsFilter>,
}
/// NewSlotExecutionOutputs Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsFilter {
    /// Filter
    #[prost(oneof = "new_slot_execution_outputs_filter::Filter", tags = "1")]
    pub filter: ::core::option::Option<new_slot_execution_outputs_filter::Filter>,
}
/// Nested message and enum types in `NewSlotExecutionOutputsFilter`.
pub mod new_slot_execution_outputs_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Execution output status
        #[prost(
            enumeration = "super::super::super::model::v1::ExecutionOutputStatus",
            tag = "1"
        )]
        Status(i32),
    }
}
/// NewSlotExecutionOutputsResponse holds response from NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsResponse {
    /// Slot execution output
    #[prost(message, optional, tag = "1")]
    pub output: ::core::option::Option<super::super::model::v1::SlotExecutionOutput>,
}
/// SendBlocksRequest holds parameters to SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksRequest {
    /// Secure shared block
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::super::model::v1::SecureShare>,
}
/// SendBlocksResponse holds response from SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksResponse {
    /// Block result or a gRPC status
    #[prost(oneof = "send_blocks_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<send_blocks_response::Result>,
}
/// Nested message and enum types in `SendBlocksResponse`.
pub mod send_blocks_response {
    /// Block result or a gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Block result
        #[prost(string, tag = "1")]
        BlockId(::prost::alloc::string::String),
        /// Massa error
        #[prost(message, tag = "2")]
        Error(super::super::super::model::v1::Error),
    }
}
/// SendEndorsementsRequest holds parameters to SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsRequest {
    /// Secure shared endorsements
    #[prost(message, repeated, tag = "1")]
    pub endorsements: ::prost::alloc::vec::Vec<super::super::model::v1::SecureShare>,
}
/// SendEndorsementsResponse holds response from SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsResponse {
    /// Endorsement result or gRPC status
    #[prost(oneof = "send_endorsements_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<send_endorsements_response::Result>,
}
/// Nested message and enum types in `SendEndorsementsResponse`.
pub mod send_endorsements_response {
    /// Endorsement result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Endorsement result
        #[prost(message, tag = "1")]
        EndorsementsIds(super::super::super::model::v1::EndorsementsIds),
        /// Massa error
        #[prost(message, tag = "2")]
        Error(super::super::super::model::v1::Error),
    }
}
/// SendOperationsRequest holds parameters to SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsRequest {
    /// Secured shared operations
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<super::super::model::v1::SecureShare>,
}
/// SendOperationsResponse holds response from SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsResponse {
    /// Operation result or gRPC status
    #[prost(oneof = "send_operations_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<send_operations_response::Result>,
}
/// Nested message and enum types in `SendOperationsResponse`.
pub mod send_operations_response {
    /// Operation result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Operation result
        #[prost(message, tag = "1")]
        OperationsIds(super::super::super::model::v1::OperationIds),
        /// Massa error
        #[prost(message, tag = "2")]
        Error(super::super::super::model::v1::Error),
    }
}
/// TransactionsThroughputRequest holds request for TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputRequest {
    /// Timer interval in seconds (Optional). Defaults to 10s
    #[prost(message, optional, tag = "1")]
    pub interval: ::core::option::Option<u64>,
}
/// TransactionsThroughputResponse holds response from TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputResponse {
    /// Transactions throughput per second
    #[prost(uint32, tag = "1")]
    pub throughput: u32,
}
/// Execution status of an operation or denunciation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionQueryExecutionStatus {
    /// Default enum value
    Unspecified = 0,
    /// The operation or denunciation was executed recently with success
    AlreadyExecutedWithSuccess = 1,
    /// The operation or denunciation was executed recently with failure
    AlreadyExecutedWithFailure = 2,
    /// The operation or denunciation was not executed recently but can still be executed unless expired
    ExecutableOrExpired = 3,
}
impl ExecutionQueryExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionQueryExecutionStatus::Unspecified => {
                "EXECUTION_QUERY_EXECUTION_STATUS_UNSPECIFIED"
            }
            ExecutionQueryExecutionStatus::AlreadyExecutedWithSuccess => {
                "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_SUCCESS"
            }
            ExecutionQueryExecutionStatus::AlreadyExecutedWithFailure => {
                "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_FAILURE"
            }
            ExecutionQueryExecutionStatus::ExecutableOrExpired => {
                "EXECUTION_QUERY_EXECUTION_STATUS_EXECUTABLE_OR_EXPIRED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_QUERY_EXECUTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_SUCCESS" => {
                Some(Self::AlreadyExecutedWithSuccess)
            }
            "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_FAILURE" => {
                Some(Self::AlreadyExecutedWithFailure)
            }
            "EXECUTION_QUERY_EXECUTION_STATUS_EXECUTABLE_OR_EXPIRED" => {
                Some(Self::ExecutableOrExpired)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod public_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Massa public gRPC service
    #[derive(Debug, Clone)]
    pub struct PublicServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PublicServiceClient<tonic::transport::Channel> {
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
    impl<T> PublicServiceClient<T>
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
        ) -> PublicServiceClient<InterceptedService<T, F>>
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
            PublicServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get blocks by ids
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "GetBlocks"));
            self.inner.unary(req, path, codec).await
        }
        /// Get datastore entries
        pub async fn get_datastore_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetDatastoreEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "GetDatastoreEntries"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get Mip status
        pub async fn get_mip_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMipStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMipStatusResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetMipStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "GetMipStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// Get next block best parents
        pub async fn get_next_block_best_parents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetNextBlockBestParents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PublicService",
                        "GetNextBlockBestParents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get operations
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "GetOperations"));
            self.inner.unary(req, path, codec).await
        }
        /// Get smart contracts execution events
        pub async fn get_sc_execution_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetScExecutionEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "GetScExecutionEvents"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get selector draws
        pub async fn get_selector_draws(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetSelectorDraws",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "GetSelectorDraws"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get stakers
        pub async fn get_stakers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStakersResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetStakers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "GetStakers"));
            self.inner.unary(req, path, codec).await
        }
        /// Get status
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStatusResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "GetStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// Get transactions throughput
        pub async fn get_transactions_throughput(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/GetTransactionsThroughput",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PublicService",
                        "GetTransactionsThroughput",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query state
        pub async fn query_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStateResponse>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/QueryState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "QueryState"));
            self.inner.unary(req, path, codec).await
        }
        /// New received and produced blocks
        pub async fn new_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::NewBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewBlocksResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/NewBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "NewBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced endorsements
        pub async fn new_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewEndorsementsResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/NewEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "NewEndorsements"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced blocks with operations
        pub async fn new_filled_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewFilledBlocksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewFilledBlocksResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/NewFilledBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "NewFilledBlocks"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced operations
        pub async fn new_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewOperationsResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/NewOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "NewOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and slot execution events
        pub async fn new_slot_execution_outputs(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewSlotExecutionOutputsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::NewSlotExecutionOutputsResponse>,
            >,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/NewSlotExecutionOutputs",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PublicService",
                        "NewSlotExecutionOutputs",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send blocks
        pub async fn send_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendBlocksResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/SendBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "SendBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// Send endorsements
        pub async fn send_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendEndorsementsResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/SendEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "SendEndorsements"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send operations
        pub async fn send_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendOperationsResponse>>,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/SendOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "SendOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// Transactions throughput
        pub async fn transactions_throughput(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::TransactionsThroughputRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::TransactionsThroughputResponse>,
            >,
            tonic::Status,
        > {
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
                "/massa.api.v1.PublicService/TransactionsThroughput",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PublicService",
                        "TransactionsThroughput",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod public_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PublicServiceServer.
    #[async_trait]
    pub trait PublicService: Send + Sync + 'static {
        /// Get blocks by ids
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        >;
        /// Get datastore entries
        async fn get_datastore_entries(
            &self,
            request: tonic::Request<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        >;
        /// Get Mip status
        async fn get_mip_status(
            &self,
            request: tonic::Request<super::GetMipStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMipStatusResponse>,
            tonic::Status,
        >;
        /// Get next block best parents
        async fn get_next_block_best_parents(
            &self,
            request: tonic::Request<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        >;
        /// Get operations
        async fn get_operations(
            &self,
            request: tonic::Request<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        >;
        /// Get smart contracts execution events
        async fn get_sc_execution_events(
            &self,
            request: tonic::Request<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        >;
        /// Get selector draws
        async fn get_selector_draws(
            &self,
            request: tonic::Request<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        >;
        /// Get stakers
        async fn get_stakers(
            &self,
            request: tonic::Request<super::GetStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStakersResponse>,
            tonic::Status,
        >;
        /// Get status
        async fn get_status(
            &self,
            request: tonic::Request<super::GetStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStatusResponse>,
            tonic::Status,
        >;
        /// Get transactions throughput
        async fn get_transactions_throughput(
            &self,
            request: tonic::Request<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        >;
        /// Query state
        async fn query_state(
            &self,
            request: tonic::Request<super::QueryStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStateResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewBlocks method.
        type NewBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks
        async fn new_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::NewBlocksStream>, tonic::Status>;
        /// Server streaming response type for the NewEndorsements method.
        type NewEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<super::NewEndorsementsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced endorsements
        async fn new_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewFilledBlocks method.
        type NewFilledBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewFilledBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks with operations
        async fn new_filled_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewFilledBlocksRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewFilledBlocksStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewOperations method.
        type NewOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::NewOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced operations
        async fn new_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewSlotExecutionOutputs method.
        type NewSlotExecutionOutputsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::NewSlotExecutionOutputsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// New received and slot execution events
        async fn new_slot_execution_outputs(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::NewSlotExecutionOutputsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendBlocks method.
        type SendBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::SendBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send blocks
        async fn send_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SendBlocksStream>, tonic::Status>;
        /// Server streaming response type for the SendEndorsements method.
        type SendEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::SendEndorsementsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Send endorsements
        async fn send_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendOperations method.
        type SendOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::SendOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send operations
        async fn send_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the TransactionsThroughput method.
        type TransactionsThroughputStream: futures_core::Stream<
                Item = std::result::Result<
                    super::TransactionsThroughputResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Transactions throughput
        async fn transactions_throughput(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::TransactionsThroughputRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::TransactionsThroughputStream>,
            tonic::Status,
        >;
    }
    /// Massa public gRPC service
    #[derive(Debug)]
    pub struct PublicServiceServer<T: PublicService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PublicService> PublicServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PublicServiceServer<T>
    where
        T: PublicService,
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
                "/massa.api.v1.PublicService/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetBlocksRequest>
                    for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_blocks(request).await };
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
                        let method = GetBlocksSvc(inner);
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
                "/massa.api.v1.PublicService/GetDatastoreEntries" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatastoreEntriesSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetDatastoreEntriesRequest>
                    for GetDatastoreEntriesSvc<T> {
                        type Response = super::GetDatastoreEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatastoreEntriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_datastore_entries(request).await
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
                        let method = GetDatastoreEntriesSvc(inner);
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
                "/massa.api.v1.PublicService/GetMipStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetMipStatusSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetMipStatusRequest>
                    for GetMipStatusSvc<T> {
                        type Response = super::GetMipStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMipStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_mip_status(request).await
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
                        let method = GetMipStatusSvc(inner);
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
                "/massa.api.v1.PublicService/GetNextBlockBestParents" => {
                    #[allow(non_camel_case_types)]
                    struct GetNextBlockBestParentsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetNextBlockBestParentsRequest>
                    for GetNextBlockBestParentsSvc<T> {
                        type Response = super::GetNextBlockBestParentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetNextBlockBestParentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_next_block_best_parents(request).await
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
                        let method = GetNextBlockBestParentsSvc(inner);
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
                "/massa.api.v1.PublicService/GetOperations" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetOperationsRequest>
                    for GetOperationsSvc<T> {
                        type Response = super::GetOperationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOperationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_operations(request).await
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
                        let method = GetOperationsSvc(inner);
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
                "/massa.api.v1.PublicService/GetScExecutionEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetScExecutionEventsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetScExecutionEventsRequest>
                    for GetScExecutionEventsSvc<T> {
                        type Response = super::GetScExecutionEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetScExecutionEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_sc_execution_events(request).await
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
                        let method = GetScExecutionEventsSvc(inner);
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
                "/massa.api.v1.PublicService/GetSelectorDraws" => {
                    #[allow(non_camel_case_types)]
                    struct GetSelectorDrawsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetSelectorDrawsRequest>
                    for GetSelectorDrawsSvc<T> {
                        type Response = super::GetSelectorDrawsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSelectorDrawsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_selector_draws(request).await
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
                        let method = GetSelectorDrawsSvc(inner);
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
                "/massa.api.v1.PublicService/GetStakers" => {
                    #[allow(non_camel_case_types)]
                    struct GetStakersSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetStakersRequest>
                    for GetStakersSvc<T> {
                        type Response = super::GetStakersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStakersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_stakers(request).await };
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
                        let method = GetStakersSvc(inner);
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
                "/massa.api.v1.PublicService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetStatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::GetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_status(request).await };
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
                        let method = GetStatusSvc(inner);
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
                "/massa.api.v1.PublicService/GetTransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsThroughputSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<
                        super::GetTransactionsThroughputRequest,
                    > for GetTransactionsThroughputSvc<T> {
                        type Response = super::GetTransactionsThroughputResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsThroughputRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_throughput(request).await
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
                        let method = GetTransactionsThroughputSvc(inner);
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
                "/massa.api.v1.PublicService/QueryState" => {
                    #[allow(non_camel_case_types)]
                    struct QueryStateSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::QueryStateRequest>
                    for QueryStateSvc<T> {
                        type Response = super::QueryStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryStateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_state(request).await };
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
                        let method = QueryStateSvc(inner);
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
                "/massa.api.v1.PublicService/NewBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::NewBlocksRequest>
                    for NewBlocksSvc<T> {
                        type Response = super::NewBlocksResponse;
                        type ResponseStream = T::NewBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_blocks(request).await };
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
                        let method = NewBlocksSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/NewEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct NewEndorsementsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::NewEndorsementsRequest>
                    for NewEndorsementsSvc<T> {
                        type Response = super::NewEndorsementsResponse;
                        type ResponseStream = T::NewEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_endorsements(request).await
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
                        let method = NewEndorsementsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/NewFilledBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewFilledBlocksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::NewFilledBlocksRequest>
                    for NewFilledBlocksSvc<T> {
                        type Response = super::NewFilledBlocksResponse;
                        type ResponseStream = T::NewFilledBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewFilledBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_filled_blocks(request).await
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
                        let method = NewFilledBlocksSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/NewOperations" => {
                    #[allow(non_camel_case_types)]
                    struct NewOperationsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::NewOperationsRequest>
                    for NewOperationsSvc<T> {
                        type Response = super::NewOperationsResponse;
                        type ResponseStream = T::NewOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_operations(request).await
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
                        let method = NewOperationsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/NewSlotExecutionOutputs" => {
                    #[allow(non_camel_case_types)]
                    struct NewSlotExecutionOutputsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<
                        super::NewSlotExecutionOutputsRequest,
                    > for NewSlotExecutionOutputsSvc<T> {
                        type Response = super::NewSlotExecutionOutputsResponse;
                        type ResponseStream = T::NewSlotExecutionOutputsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_slot_execution_outputs(request).await
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
                        let method = NewSlotExecutionOutputsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/SendBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct SendBlocksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::SendBlocksRequest>
                    for SendBlocksSvc<T> {
                        type Response = super::SendBlocksResponse;
                        type ResponseStream = T::SendBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).send_blocks(request).await };
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
                        let method = SendBlocksSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/SendEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct SendEndorsementsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::SendEndorsementsRequest>
                    for SendEndorsementsSvc<T> {
                        type Response = super::SendEndorsementsResponse;
                        type ResponseStream = T::SendEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_endorsements(request).await
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
                        let method = SendEndorsementsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/SendOperations" => {
                    #[allow(non_camel_case_types)]
                    struct SendOperationsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::SendOperationsRequest>
                    for SendOperationsSvc<T> {
                        type Response = super::SendOperationsResponse;
                        type ResponseStream = T::SendOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_operations(request).await
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
                        let method = SendOperationsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.PublicService/TransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionsThroughputSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<
                        super::TransactionsThroughputRequest,
                    > for TransactionsThroughputSvc<T> {
                        type Response = super::TransactionsThroughputResponse;
                        type ResponseStream = T::TransactionsThroughputStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::TransactionsThroughputRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).transactions_throughput(request).await
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
                        let method = TransactionsThroughputSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
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
    impl<T: PublicService> Clone for PublicServiceServer<T> {
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
    impl<T: PublicService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PublicService> tonic::server::NamedService for PublicServiceServer<T> {
        const NAME: &'static str = "massa.api.v1.PublicService";
    }
}
