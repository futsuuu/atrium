// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.identity.updateHandle` namespace.

/// Updates the handle of the account
#[async_trait::async_trait]
pub trait UpdateHandle: crate::xrpc::XrpcClient {
    async fn update_handle(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.identity.updateHandle",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub handle: String,
}

pub enum Error {
}
