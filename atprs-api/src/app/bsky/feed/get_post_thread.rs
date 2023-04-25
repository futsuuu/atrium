// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.getPostThread` namespace.

#[async_trait::async_trait]
pub trait GetPostThread: crate::xrpc::XrpcClient {
    async fn get_post_thread(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "app.bsky.feed.getPostThread",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub depth: Option<i32>,
    pub uri: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    // pub thread: ...,
}

pub enum Error {
    NotFound,
}
