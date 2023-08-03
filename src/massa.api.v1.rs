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
                                (*inner).ban_nodes_by_ids(request).await
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
                                (*inner).ban_nodes_by_ips(request).await
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
                                (*inner).unban_nodes_by_ids(request).await
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
                                (*inner).unban_nodes_by_ips(request).await
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
