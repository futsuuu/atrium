// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.server.createSession` namespace.

/// Create an authentication session.
#[async_trait::async_trait]
pub trait CreateSession: crate::xrpc::XrpcClient {
    async fn create_session(&self, input: Input) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.server.createSession",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// Handle or other identifier supported by the server for the authenticating user.
    pub identifier: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub access_jwt: String,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub handle: String,
    pub refresh_jwt: String,
}

pub enum Error {
    AccountTakedown,
}
