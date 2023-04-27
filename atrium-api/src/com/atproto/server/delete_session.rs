// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.server.deleteSession` namespace.

/// Delete the current session.
#[async_trait::async_trait]
pub trait DeleteSession: crate::xrpc::XrpcClient {
    async fn delete_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.server.deleteSession",
            Option::<()>::None,
            Option::<()>::None,
        )
        .await
    }
}

pub enum Error {
}