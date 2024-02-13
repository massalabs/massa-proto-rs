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
/// AllowEveryoneToBootstrapRequest holds the request for AllowEveryoneToBootstrap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowEveryoneToBootstrapRequest {}
/// AllowEveryoneToBootstrapResponse holds the response from AllowEveryoneToBootstrap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowEveryoneToBootstrapResponse {}
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
/// BanNodesByIdsRequest holds the request for BanNodesByIds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanNodesByIdsRequest {
    /// Node ids to ban
    #[prost(string, repeated, tag = "1")]
    pub node_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BanNodesByIdsResponse holds the response from BanNodesByIds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanNodesByIdsResponse {}
/// BanNodesByIpsRequest holds the request for BanNodesByIps
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanNodesByIpsRequest {
    /// Node IP addresses to ban
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BanNodesByIpsResponse holds the response from BanNodesByIps
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanNodesByIpsResponse {}
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
/// UnbanNodesByIdsRequest holds the request for UnbanNodesByIds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbanNodesByIdsRequest {
    /// Node ids to unban
    #[prost(string, repeated, tag = "1")]
    pub node_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UnbanNodesByIdsResponse holds the response from UnbanNodesByIds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbanNodesByIdsResponse {}
/// UnbanNodesByIpsRequest holds the request for UnbanNodesByIps
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbanNodesByIpsRequest {
    /// Nodes IP addresses to unban
    #[prost(string, repeated, tag = "1")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UnbanNodesByIpsResponse holds the response from UnbanNodesByIps
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbanNodesByIpsResponse {}
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
                "/massa.api.v1.PrivateService/GetMipStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PrivateService", "GetMipStatus"));
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
        /// Ban multiple nodes by their individual ids
        pub async fn ban_nodes_by_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::BanNodesByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BanNodesByIdsResponse>,
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
                "/massa.api.v1.PrivateService/BanNodesByIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PrivateService", "BanNodesByIds"));
            self.inner.unary(req, path, codec).await
        }
        /// Ban multiple nodes by their individual IP addresses
        pub async fn ban_nodes_by_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::BanNodesByIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BanNodesByIpsResponse>,
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
                "/massa.api.v1.PrivateService/BanNodesByIps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PrivateService", "BanNodesByIps"));
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
        /// Unban multiple nodes by their individual ids
        pub async fn unban_nodes_by_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::UnbanNodesByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnbanNodesByIdsResponse>,
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
                "/massa.api.v1.PrivateService/UnbanNodesByIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PrivateService", "UnbanNodesByIds"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Unban multiple nodes by their individual IP addresses
        pub async fn unban_nodes_by_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::UnbanNodesByIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnbanNodesByIpsResponse>,
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
                "/massa.api.v1.PrivateService/UnbanNodesByIps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PrivateService", "UnbanNodesByIps"),
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
        /// Allow everyone to bootstrap from the node by removing bootstrap whitelist configuration file
        async fn allow_everyone_to_bootstrap(
            &self,
            request: tonic::Request<super::AllowEveryoneToBootstrapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllowEveryoneToBootstrapResponse>,
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
        /// Get Mip status
        async fn get_mip_status(
            &self,
            request: tonic::Request<super::GetMipStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMipStatusResponse>,
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
        /// Ban multiple nodes by their individual ids
        async fn ban_nodes_by_ids(
            &self,
            request: tonic::Request<super::BanNodesByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BanNodesByIdsResponse>,
            tonic::Status,
        >;
        /// Ban multiple nodes by their individual IP addresses
        async fn ban_nodes_by_ips(
            &self,
            request: tonic::Request<super::BanNodesByIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BanNodesByIpsResponse>,
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
        /// Unban multiple nodes by their individual ids
        async fn unban_nodes_by_ids(
            &self,
            request: tonic::Request<super::UnbanNodesByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnbanNodesByIdsResponse>,
            tonic::Status,
        >;
        /// Unban multiple nodes by their individual IP addresses
        async fn unban_nodes_by_ips(
            &self,
            request: tonic::Request<super::UnbanNodesByIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnbanNodesByIpsResponse>,
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
                                <T as PrivateService>::add_to_bootstrap_blacklist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::add_to_bootstrap_whitelist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::add_to_peers_whitelist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::add_staking_secret_keys(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::allow_everyone_to_bootstrap(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::get_bootstrap_blacklist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::get_bootstrap_whitelist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                "/massa.api.v1.PrivateService/GetMipStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetMipStatusSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
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
                                <T as PrivateService>::get_mip_status(&inner, request).await
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
                                <T as PrivateService>::get_node_status(&inner, request)
                                    .await
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
                                <T as PrivateService>::get_peers_whitelist(&inner, request)
                                    .await
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
                "/massa.api.v1.PrivateService/BanNodesByIds" => {
                    #[allow(non_camel_case_types)]
                    struct BanNodesByIdsSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::BanNodesByIdsRequest>
                    for BanNodesByIdsSvc<T> {
                        type Response = super::BanNodesByIdsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BanNodesByIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivateService>::ban_nodes_by_ids(&inner, request)
                                    .await
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
                        let method = BanNodesByIdsSvc(inner);
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
                "/massa.api.v1.PrivateService/BanNodesByIps" => {
                    #[allow(non_camel_case_types)]
                    struct BanNodesByIpsSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::BanNodesByIpsRequest>
                    for BanNodesByIpsSvc<T> {
                        type Response = super::BanNodesByIpsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BanNodesByIpsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivateService>::ban_nodes_by_ips(&inner, request)
                                    .await
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
                        let method = BanNodesByIpsSvc(inner);
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
                                <T as PrivateService>::remove_from_bootstrap_blacklist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::remove_from_bootstrap_whitelist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::remove_from_peers_whitelist(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::remove_staking_addresses(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PrivateService>::sign_messages(&inner, request).await
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
                                <T as PrivateService>::shutdown_gracefully(&inner, request)
                                    .await
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
                "/massa.api.v1.PrivateService/UnbanNodesByIds" => {
                    #[allow(non_camel_case_types)]
                    struct UnbanNodesByIdsSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::UnbanNodesByIdsRequest>
                    for UnbanNodesByIdsSvc<T> {
                        type Response = super::UnbanNodesByIdsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnbanNodesByIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivateService>::unban_nodes_by_ids(&inner, request)
                                    .await
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
                        let method = UnbanNodesByIdsSvc(inner);
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
                "/massa.api.v1.PrivateService/UnbanNodesByIps" => {
                    #[allow(non_camel_case_types)]
                    struct UnbanNodesByIpsSvc<T: PrivateService>(pub Arc<T>);
                    impl<
                        T: PrivateService,
                    > tonic::server::UnaryService<super::UnbanNodesByIpsRequest>
                    for UnbanNodesByIpsSvc<T> {
                        type Response = super::UnbanNodesByIpsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnbanNodesByIpsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrivateService>::unban_nodes_by_ips(&inner, request)
                                    .await
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
                        let method = UnbanNodesByIpsSvc(inner);
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
/// ExecuteReadOnlyCallRequest holds request for ExecuteReadOnlyCall
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadOnlyCallRequest {
    /// Execution call
    #[prost(message, optional, tag = "1")]
    pub call: ::core::option::Option<super::super::model::v1::ReadOnlyExecutionCall>,
}
/// ExecuteReadOnlyCallResponse holds response from ExecuteReadOnlyCall
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadOnlyCallResponse {
    /// Execution output
    #[prost(message, optional, tag = "1")]
    pub output: ::core::option::Option<super::super::model::v1::ReadOnlyExecutionOutput>,
}
/// GetBlocksRequest holds request for GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    /// Block ids
    #[prost(string, repeated, tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        /// One of these (address-key) pairs
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
/// GetEndorsementsRequest holds request for GetEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndorsementsRequest {
    /// Endorsement ids
    #[prost(string, repeated, tag = "1")]
    pub endorsement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetEndorsementsResponse holds response from GetEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndorsementsResponse {
    /// Wrapped operations
    #[prost(message, repeated, tag = "1")]
    pub wrapped_endorsements: ::prost::alloc::vec::Vec<
        super::super::model::v1::EndorsementWrapper,
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
    /// Operation ids
    #[prost(string, repeated, tag = "1")]
    pub operation_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    /// Filter
    #[prost(oneof = "sc_execution_events_filter::Filter", tags = "1, 2, 3, 4, 5, 6")]
    pub filter: ::core::option::Option<sc_execution_events_filter::Filter>,
}
/// Nested message and enum types in `ScExecutionEventsFilter`.
pub mod sc_execution_events_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Slot range
        #[prost(message, tag = "1")]
        SlotRange(super::super::super::model::v1::SlotRange),
        /// Caller address
        #[prost(string, tag = "2")]
        CallerAddress(::prost::alloc::string::String),
        /// Emitter address
        #[prost(string, tag = "3")]
        EmitterAddress(::prost::alloc::string::String),
        /// Original operation id
        #[prost(string, tag = "4")]
        OriginalOperationId(::prost::alloc::string::String),
        /// Whether the event is a failure
        #[prost(bool, tag = "5")]
        IsFailure(bool),
        /// Status
        #[prost(
            enumeration = "super::super::super::model::v1::ScExecutionEventStatus",
            tag = "6"
        )]
        Status(i32),
    }
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
        /// Minimum rolls
        #[prost(uint64, tag = "1")]
        MinRolls(u64),
        /// Maximum rolls
        #[prost(uint64, tag = "2")]
        MaxRolls(u64),
        /// Limit
        #[prost(uint64, tag = "3")]
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
    /// Execution queries
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<ExecutionQueryRequestItem>,
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
    /// Returns all the events that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<ScExecutionEventsFilter>,
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
    /// The hash of the XOF final state hash
    #[prost(string, tag = "3")]
    pub final_state_fingerprint: ::prost::alloc::string::String,
    /// List of execution query response items
    #[prost(message, repeated, tag = "4")]
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
        #[prost(message, tag = "6")]
        DeferredCredits(super::DeferredCreditsEntryWrapper),
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
/// Deferred credits entry wrapper
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCreditsEntryWrapper {
    /// Deferred credits entry
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<DeferredCreditsEntry>,
}
/// Deferred credits entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCreditsEntry {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
    /// Amount
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::model::v1::NativeAmount>,
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
    #[prost(message, optional, tag = "2")]
    pub production_stats: ::core::option::Option<
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
/// ScOutputEvents wrapper
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScOutputEventsWrapper {
    /// Events
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::super::model::v1::ScExecutionEvent>,
}
/// NewBlocksRequest holds request for NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksRequest {
    /// Returns all the blocks that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<NewBlocksFilter>,
}
/// NewBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksFilter {
    /// Filter
    #[prost(oneof = "new_blocks_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<new_blocks_filter::Filter>,
}
/// Nested message and enum types in `NewBlocksFilter`.
pub mod new_blocks_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of these block ids
        #[prost(message, tag = "1")]
        BlockIds(super::super::super::model::v1::BlockIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of these slot ranges (inclusive)
        #[prost(message, tag = "3")]
        SlotRange(super::super::super::model::v1::SlotRange),
    }
}
/// NewBlocksResponse holds response from NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksResponse {
    /// Signed block
    #[prost(message, optional, tag = "1")]
    pub signed_block: ::core::option::Option<super::super::model::v1::SignedBlock>,
}
/// NewEndorsementsRequest holds request for NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsRequest {
    /// Returns all the endorsements that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<NewEndorsementsFilter>,
}
/// NewEndorsements Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsFilter {
    /// Filter
    #[prost(oneof = "new_endorsements_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<new_endorsements_filter::Filter>,
}
/// Nested message and enum types in `NewEndorsementsFilter`.
pub mod new_endorsements_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of these endorsement ids
        #[prost(message, tag = "1")]
        EndorsementIds(super::super::super::model::v1::EndorsementIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of these block ids
        #[prost(message, tag = "3")]
        BlockIds(super::super::super::model::v1::BlockIds),
    }
}
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
pub struct NewFilledBlocksRequest {
    /// Returns all the blocks that verify one of the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<NewBlocksFilter>,
}
/// NewFilledBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksFilter {
    /// Filter
    #[prost(oneof = "new_filled_blocks_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<new_filled_blocks_filter::Filter>,
}
/// Nested message and enum types in `NewFilledBlocksFilter`.
pub mod new_filled_blocks_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of these block ids
        #[prost(message, tag = "1")]
        BlockIds(super::super::super::model::v1::BlockIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of these slot ranges (inclusive)
        #[prost(message, tag = "3")]
        SlotRange(super::super::super::model::v1::SlotRange),
    }
}
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
    #[prost(oneof = "new_operations_filter::Filter", tags = "1, 2, 3")]
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
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of the operation types
        #[prost(message, tag = "3")]
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
    #[prost(
        oneof = "new_slot_execution_outputs_filter::Filter",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
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
        /// Slot range
        #[prost(message, tag = "2")]
        SlotRange(super::super::super::model::v1::SlotRange),
        /// AsyncPoolChangesFilter
        #[prost(message, tag = "3")]
        AsyncPoolChangesFilter(super::AsyncPoolChangesFilter),
        /// ExecutedDenounciationFilter
        #[prost(message, tag = "4")]
        ExecutedDenounciationFilter(super::ExecutedDenounciationFilter),
        /// Execution event filter
        #[prost(message, tag = "5")]
        EventFilter(super::ExecutionEventFilter),
        /// ExecutedOpsChangesFilter
        #[prost(message, tag = "6")]
        ExecutedOpsChangesFilter(super::ExecutedOpsChangesFilter),
        /// LedgerChangesFilter
        #[prost(message, tag = "7")]
        LedgerChangesFilter(super::LedgerChangesFilter),
    }
}
/// AsyncPoolChangesFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangesFilter {
    /// Filter
    #[prost(oneof = "async_pool_changes_filter::Filter", tags = "1, 2, 3, 4, 5, 6")]
    pub filter: ::core::option::Option<async_pool_changes_filter::Filter>,
}
/// Nested message and enum types in `AsyncPoolChangesFilter`.
pub mod async_pool_changes_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
        /// The type of the change
        #[prost(
            enumeration = "super::super::super::model::v1::AsyncPoolChangeType",
            tag = "2"
        )]
        Type(i32),
        /// The handler function name within the destination address bytecode
        #[prost(string, tag = "3")]
        Handler(::prost::alloc::string::String),
        /// The address towards which the message is being sent
        #[prost(string, tag = "4")]
        DestinationAddress(::prost::alloc::string::String),
        /// The address that sent the message
        #[prost(string, tag = "5")]
        EmitterAddress(::prost::alloc::string::String),
        /// Boolean that determine if the message can be executed. For messages without filter this boolean is always true.
        /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
        #[prost(bool, tag = "6")]
        CanBeExecuted(bool),
    }
}
/// PosChangesFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosChangesFilter {
    /// Filter
    #[prost(oneof = "pos_changes_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<pos_changes_filter::Filter>,
}
/// Nested message and enum types in `PosChangesFilter`.
pub mod pos_changes_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
        /// Address for which we have roll changes
        #[prost(string, tag = "2")]
        Address(::prost::alloc::string::String),
    }
}
/// ExecutionEventFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionEventFilter {
    /// Filter
    #[prost(oneof = "execution_event_filter::Filter", tags = "1, 2, 3, 4, 5")]
    pub filter: ::core::option::Option<execution_event_filter::Filter>,
}
/// Nested message and enum types in `ExecutionEventFilter`.
pub mod execution_event_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
        /// Caller address
        #[prost(string, tag = "2")]
        CallerAddress(::prost::alloc::string::String),
        /// Emitter address
        #[prost(string, tag = "3")]
        EmitterAddress(::prost::alloc::string::String),
        /// Original operation id
        #[prost(string, tag = "4")]
        OriginalOperationId(::prost::alloc::string::String),
        /// Whether the event is a failure
        #[prost(bool, tag = "5")]
        IsFailure(bool),
    }
}
/// ExecutedOpsChangesFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangesFilter {
    /// Filter
    #[prost(oneof = "executed_ops_changes_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<executed_ops_changes_filter::Filter>,
}
/// Nested message and enum types in `ExecutedOpsChangesFilter`.
pub mod executed_ops_changes_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
        /// Operation id
        #[prost(string, tag = "2")]
        OperationId(::prost::alloc::string::String),
    }
}
/// ExecutedDenounciationFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedDenounciationFilter {
    /// Filter
    #[prost(oneof = "executed_denounciation_filter::Filter", tags = "1")]
    pub filter: ::core::option::Option<executed_denounciation_filter::Filter>,
}
/// Nested message and enum types in `ExecutedDenounciationFilter`.
pub mod executed_denounciation_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
    }
}
/// LedgerChangesFilter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangesFilter {
    /// Filter
    #[prost(oneof = "ledger_changes_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<ledger_changes_filter::Filter>,
}
/// Nested message and enum types in `LedgerChangesFilter`.
pub mod ledger_changes_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Do not return any message
        #[prost(message, tag = "1")]
        None(super::super::super::model::v1::Empty),
        /// Address for which we have ledger changes
        #[prost(string, tag = "2")]
        Address(::prost::alloc::string::String),
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
/// NewSlotABICallStacks request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotAbiCallStacksRequest {
    /// Finality level to receive informations from
    #[prost(enumeration = "FinalityLevel", tag = "1")]
    pub finality_level: i32,
}
/// NewSlotABICallStacks response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotAbiCallStacksResponse {
    /// Finality level to receive informations from
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
    /// Call stacks for asynchronous execution
    #[prost(message, repeated, tag = "2")]
    pub asc_call_stacks: ::prost::alloc::vec::Vec<AscabiCallStack>,
    /// Call stack for operations
    #[prost(message, repeated, tag = "3")]
    pub operation_call_stacks: ::prost::alloc::vec::Vec<OperationAbiCallStack>,
}
/// SendBlocksRequest holds parameters to SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksRequest {
    /// Secure shared block
    #[prost(bytes = "vec", tag = "1")]
    pub block: ::prost::alloc::vec::Vec<u8>,
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
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub endorsements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
        EndorsementIds(super::super::super::model::v1::EndorsementIds),
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
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
        OperationIds(super::super::super::model::v1::OperationIds),
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
/// SearchBlocksRequest holds request for SearchBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBlocksRequest {
    /// Returns all the blocks that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<SearchBlocksFilter>,
}
/// SearchBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBlocksFilter {
    /// Filter
    #[prost(oneof = "search_blocks_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<search_blocks_filter::Filter>,
}
/// Nested message and enum types in `SearchBlocksFilter`.
pub mod search_blocks_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of these block ids
        #[prost(message, tag = "1")]
        BlockIds(super::super::super::model::v1::BlockIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of these slot ranges (inclusive)
        #[prost(message, tag = "3")]
        SlotRange(super::super::super::model::v1::SlotRange),
    }
}
/// SearchBlocksResponse holds response from SearchBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBlocksResponse {
    /// Information about the blocks
    #[prost(message, repeated, tag = "1")]
    pub block_infos: ::prost::alloc::vec::Vec<super::super::model::v1::BlockInfo>,
}
/// SearchEndorsementsRequest holds request for SearchEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEndorsementsRequest {
    /// Returns all the endorsements informations that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<SearchEndorsementsFilter>,
}
/// SearchEndorsements Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEndorsementsFilter {
    /// Filter
    #[prost(oneof = "search_endorsements_filter::Filter", tags = "1, 2, 3")]
    pub filter: ::core::option::Option<search_endorsements_filter::Filter>,
}
/// Nested message and enum types in `SearchEndorsementsFilter`.
pub mod search_endorsements_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of these endorsement ids
        #[prost(message, tag = "1")]
        EndorsementIds(super::super::super::model::v1::EndorsementIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
        /// One of these block ids
        #[prost(message, tag = "3")]
        BlockIds(super::super::super::model::v1::BlockIds),
    }
}
/// SearchEndorsementsResponse holds response from SearchEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEndorsementsResponse {
    /// Information about the endorsements
    #[prost(message, repeated, tag = "1")]
    pub endorsement_infos: ::prost::alloc::vec::Vec<
        super::super::model::v1::EndorsementInfo,
    >,
}
/// SearchOperationsRequest holds request for SearchOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOperationsRequest {
    /// Returns all the operations that verify all the filters
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<SearchOperationsFilter>,
}
/// SearchOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOperationsFilter {
    /// Filter
    #[prost(oneof = "search_operations_filter::Filter", tags = "1, 2")]
    pub filter: ::core::option::Option<search_operations_filter::Filter>,
}
/// Nested message and enum types in `SearchOperationsFilter`.
pub mod search_operations_filter {
    /// Filter
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// One of the operation ids
        #[prost(message, tag = "1")]
        OperationIds(super::super::super::model::v1::OperationIds),
        /// One of these creator addresses
        #[prost(message, tag = "2")]
        Addresses(super::super::super::model::v1::Addresses),
    }
}
/// SearchOperationsResponse holds response from SearchOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOperationsResponse {
    /// Information about the operations
    #[prost(message, repeated, tag = "1")]
    pub operation_infos: ::prost::alloc::vec::Vec<
        super::super::model::v1::OperationInfo,
    >,
}
/// GetOperationABICallStacks request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationAbiCallStacksRequest {
    /// Operations ids to get the call stack from
    #[prost(string, repeated, tag = "1")]
    pub operation_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Definition of an ABI call stack element
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiCallStackElement {
    /// name of the ABI
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Parameters of the ABI
    #[prost(string, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Return value of the ABI
    #[prost(string, tag = "3")]
    pub return_value: ::prost::alloc::string::String,
}
/// Definition of an ABI call stack element that is the 'call' ABI
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiCallStackElementCall {
    /// name of the ABI
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Parameters of the ABI
    #[prost(string, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Calls made within this SC call
    #[prost(message, repeated, tag = "3")]
    pub sub_calls: ::prost::alloc::vec::Vec<AbiCallStackElementParent>,
    /// Return value of the ABI
    #[prost(string, tag = "4")]
    pub return_value: ::prost::alloc::string::String,
}
/// Definition of an ABI call stack element parent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiCallStackElementParent {
    /// Element of the call stack
    #[prost(oneof = "abi_call_stack_element_parent::CallStackElement", tags = "1, 2")]
    pub call_stack_element: ::core::option::Option<
        abi_call_stack_element_parent::CallStackElement,
    >,
}
/// Nested message and enum types in `ABICallStackElementParent`.
pub mod abi_call_stack_element_parent {
    /// Element of the call stack
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CallStackElement {
        /// Any ABI call that is not the ABI 'call'
        #[prost(message, tag = "1")]
        Element(super::AbiCallStackElement),
        /// Element that is the ABI 'call'
        #[prost(message, tag = "2")]
        ElementCall(super::AbiCallStackElementCall),
    }
}
/// Definition of an ABI call stack
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiCallStack {
    /// All elements of the call stack
    #[prost(message, repeated, tag = "1")]
    pub call_stack: ::prost::alloc::vec::Vec<AbiCallStackElementParent>,
}
/// GetOperationABICallStacks response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationAbiCallStacksResponse {
    #[prost(message, repeated, tag = "1")]
    pub call_stacks: ::prost::alloc::vec::Vec<AbiCallStack>,
}
/// GetSlotABICallStacks request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSlotAbiCallStacksRequest {
    /// Slots asked
    #[prost(message, repeated, tag = "1")]
    pub slots: ::prost::alloc::vec::Vec<super::super::model::v1::Slot>,
}
/// ABI asynchronous execution call stack
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AscabiCallStack {
    /// Index of the execution in the slot
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// Call stack
    #[prost(message, repeated, tag = "2")]
    pub call_stack: ::prost::alloc::vec::Vec<AbiCallStackElementParent>,
}
/// Operation execution call stack
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationAbiCallStack {
    /// Operation id
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// Call stack
    #[prost(message, repeated, tag = "2")]
    pub call_stack: ::prost::alloc::vec::Vec<AbiCallStackElementParent>,
}
/// Call stack for a slot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotAbiCallStacks {
    /// Call stacks for asynchronous execution
    #[prost(message, repeated, tag = "1")]
    pub asc_call_stacks: ::prost::alloc::vec::Vec<AscabiCallStack>,
    /// Call stack for operations
    #[prost(message, repeated, tag = "2")]
    pub operation_call_stacks: ::prost::alloc::vec::Vec<OperationAbiCallStack>,
}
/// GetSlotABICallStacks response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSlotAbiCallStacksResponse {
    /// Call stacks for the slots
    #[prost(message, repeated, tag = "1")]
    pub slot_call_stacks: ::prost::alloc::vec::Vec<SlotAbiCallStacks>,
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
/// Finality level to filter on in streams
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FinalityLevel {
    /// Unspecified (receive both)
    Unspecified = 0,
    /// Candidate level
    Candidate = 1,
    /// Final level
    Final = 2,
}
impl FinalityLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FinalityLevel::Unspecified => "FINALITY_LEVEL_UNSPECIFIED",
            FinalityLevel::Candidate => "FINALITY_LEVEL_CANDIDATE",
            FinalityLevel::Final => "FINALITY_LEVEL_FINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FINALITY_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "FINALITY_LEVEL_CANDIDATE" => Some(Self::Candidate),
            "FINALITY_LEVEL_FINAL" => Some(Self::Final),
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
        /// Execute read only call
        pub async fn execute_read_only_call(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteReadOnlyCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadOnlyCallResponse>,
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
                "/massa.api.v1.PublicService/ExecuteReadOnlyCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "ExecuteReadOnlyCall"),
                );
            self.inner.unary(req, path, codec).await
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
        /// Get endorsements by ids
        pub async fn get_endorsements(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndorsementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndorsementsResponse>,
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
                "/massa.api.v1.PublicService/GetEndorsements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "GetEndorsements"),
                );
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
        /// Get operations by ids
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
        /// Search blocks
        pub async fn search_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchBlocksResponse>,
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
                "/massa.api.v1.PublicService/SearchBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.PublicService", "SearchBlocks"));
            self.inner.unary(req, path, codec).await
        }
        /// Search endorsements
        pub async fn search_endorsements(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchEndorsementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchEndorsementsResponse>,
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
                "/massa.api.v1.PublicService/SearchEndorsements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "SearchEndorsements"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Search operations
        pub async fn search_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchOperationsResponse>,
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
                "/massa.api.v1.PublicService/SearchOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "SearchOperations"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get ABI call stack of an operation
        pub async fn get_operation_abi_call_stacks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationAbiCallStacksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationAbiCallStacksResponse>,
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
                "/massa.api.v1.PublicService/GetOperationABICallStacks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.PublicService",
                        "GetOperationABICallStacks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get ABI call stack of all asynchronous executions and all operations for a given slot
        pub async fn get_slot_abi_call_stacks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSlotAbiCallStacksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSlotAbiCallStacksResponse>,
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
                "/massa.api.v1.PublicService/GetSlotABICallStacks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "GetSlotABICallStacks"),
                );
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
        /// Call stack for each slot executed
        pub async fn new_slot_abi_call_stacks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewSlotAbiCallStacksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::NewSlotAbiCallStacksResponse>,
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
                "/massa.api.v1.PublicService/NewSlotABICallStacks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.PublicService", "NewSlotABICallStacks"),
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
        /// Execute read only call
        async fn execute_read_only_call(
            &self,
            request: tonic::Request<super::ExecuteReadOnlyCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadOnlyCallResponse>,
            tonic::Status,
        >;
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
        /// Get endorsements by ids
        async fn get_endorsements(
            &self,
            request: tonic::Request<super::GetEndorsementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndorsementsResponse>,
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
        /// Get operations by ids
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
        /// Search blocks
        async fn search_blocks(
            &self,
            request: tonic::Request<super::SearchBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchBlocksResponse>,
            tonic::Status,
        >;
        /// Search endorsements
        async fn search_endorsements(
            &self,
            request: tonic::Request<super::SearchEndorsementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchEndorsementsResponse>,
            tonic::Status,
        >;
        /// Search operations
        async fn search_operations(
            &self,
            request: tonic::Request<super::SearchOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchOperationsResponse>,
            tonic::Status,
        >;
        /// Get ABI call stack of an operation
        async fn get_operation_abi_call_stacks(
            &self,
            request: tonic::Request<super::GetOperationAbiCallStacksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationAbiCallStacksResponse>,
            tonic::Status,
        >;
        /// Get ABI call stack of all asynchronous executions and all operations for a given slot
        async fn get_slot_abi_call_stacks(
            &self,
            request: tonic::Request<super::GetSlotAbiCallStacksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSlotAbiCallStacksResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewBlocks method.
        type NewBlocksStream: tonic::codegen::tokio_stream::Stream<
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
        type NewEndorsementsStream: tonic::codegen::tokio_stream::Stream<
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
        type NewFilledBlocksStream: tonic::codegen::tokio_stream::Stream<
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
        type NewOperationsStream: tonic::codegen::tokio_stream::Stream<
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
        type NewSlotExecutionOutputsStream: tonic::codegen::tokio_stream::Stream<
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
        /// Server streaming response type for the NewSlotABICallStacks method.
        type NewSlotABICallStacksStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::NewSlotAbiCallStacksResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Call stack for each slot executed
        async fn new_slot_abi_call_stacks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewSlotAbiCallStacksRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewSlotABICallStacksStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendBlocks method.
        type SendBlocksStream: tonic::codegen::tokio_stream::Stream<
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
        type SendEndorsementsStream: tonic::codegen::tokio_stream::Stream<
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
        type SendOperationsStream: tonic::codegen::tokio_stream::Stream<
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
        type TransactionsThroughputStream: tonic::codegen::tokio_stream::Stream<
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
                "/massa.api.v1.PublicService/ExecuteReadOnlyCall" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteReadOnlyCallSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::ExecuteReadOnlyCallRequest>
                    for ExecuteReadOnlyCallSvc<T> {
                        type Response = super::ExecuteReadOnlyCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteReadOnlyCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::execute_read_only_call(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = ExecuteReadOnlyCallSvc(inner);
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
                            let fut = async move {
                                <T as PublicService>::get_blocks(&inner, request).await
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
                                <T as PublicService>::get_datastore_entries(&inner, request)
                                    .await
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
                "/massa.api.v1.PublicService/GetEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct GetEndorsementsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetEndorsementsRequest>
                    for GetEndorsementsSvc<T> {
                        type Response = super::GetEndorsementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEndorsementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::get_endorsements(&inner, request)
                                    .await
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
                        let method = GetEndorsementsSvc(inner);
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
                                <T as PublicService>::get_next_block_best_parents(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PublicService>::get_operations(&inner, request).await
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
                                <T as PublicService>::get_sc_execution_events(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                                <T as PublicService>::get_selector_draws(&inner, request)
                                    .await
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
                            let fut = async move {
                                <T as PublicService>::get_stakers(&inner, request).await
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
                            let fut = async move {
                                <T as PublicService>::get_status(&inner, request).await
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
                                <T as PublicService>::get_transactions_throughput(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                            let fut = async move {
                                <T as PublicService>::query_state(&inner, request).await
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
                "/massa.api.v1.PublicService/SearchBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct SearchBlocksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::SearchBlocksRequest>
                    for SearchBlocksSvc<T> {
                        type Response = super::SearchBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::search_blocks(&inner, request).await
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
                        let method = SearchBlocksSvc(inner);
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
                "/massa.api.v1.PublicService/SearchEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct SearchEndorsementsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::SearchEndorsementsRequest>
                    for SearchEndorsementsSvc<T> {
                        type Response = super::SearchEndorsementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchEndorsementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::search_endorsements(&inner, request)
                                    .await
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
                        let method = SearchEndorsementsSvc(inner);
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
                "/massa.api.v1.PublicService/SearchOperations" => {
                    #[allow(non_camel_case_types)]
                    struct SearchOperationsSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::SearchOperationsRequest>
                    for SearchOperationsSvc<T> {
                        type Response = super::SearchOperationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchOperationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::search_operations(&inner, request)
                                    .await
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
                        let method = SearchOperationsSvc(inner);
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
                "/massa.api.v1.PublicService/GetOperationABICallStacks" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationABICallStacksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<
                        super::GetOperationAbiCallStacksRequest,
                    > for GetOperationABICallStacksSvc<T> {
                        type Response = super::GetOperationAbiCallStacksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOperationAbiCallStacksRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::get_operation_abi_call_stacks(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GetOperationABICallStacksSvc(inner);
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
                "/massa.api.v1.PublicService/GetSlotABICallStacks" => {
                    #[allow(non_camel_case_types)]
                    struct GetSlotABICallStacksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::UnaryService<super::GetSlotAbiCallStacksRequest>
                    for GetSlotABICallStacksSvc<T> {
                        type Response = super::GetSlotAbiCallStacksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSlotAbiCallStacksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::get_slot_abi_call_stacks(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GetSlotABICallStacksSvc(inner);
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
                            let fut = async move {
                                <T as PublicService>::new_blocks(&inner, request).await
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
                                <T as PublicService>::new_endorsements(&inner, request)
                                    .await
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
                                <T as PublicService>::new_filled_blocks(&inner, request)
                                    .await
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
                                <T as PublicService>::new_operations(&inner, request).await
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
                                <T as PublicService>::new_slot_execution_outputs(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                "/massa.api.v1.PublicService/NewSlotABICallStacks" => {
                    #[allow(non_camel_case_types)]
                    struct NewSlotABICallStacksSvc<T: PublicService>(pub Arc<T>);
                    impl<
                        T: PublicService,
                    > tonic::server::StreamingService<super::NewSlotAbiCallStacksRequest>
                    for NewSlotABICallStacksSvc<T> {
                        type Response = super::NewSlotAbiCallStacksResponse;
                        type ResponseStream = T::NewSlotABICallStacksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewSlotAbiCallStacksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PublicService>::new_slot_abi_call_stacks(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = NewSlotABICallStacksSvc(inner);
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
                            let fut = async move {
                                <T as PublicService>::send_blocks(&inner, request).await
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
                                <T as PublicService>::send_endorsements(&inner, request)
                                    .await
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
                                <T as PublicService>::send_operations(&inner, request).await
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
                                <T as PublicService>::transactions_throughput(
                                        &inner,
                                        request,
                                    )
                                    .await
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
