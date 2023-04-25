// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.server.refreshSession` namespace.

/// Refresh an authentication session.
#[async_trait::async_trait]
pub trait RefreshSession: crate::xrpc::XrpcClient {
    async fn refresh_session(&self) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.server.refreshSession",
            Option::<()>::None,
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub access_jwt: String,
    pub did: String,
    pub handle: String,
    pub refresh_jwt: String,
}

pub enum Error {
    AccountTakedown,
}
