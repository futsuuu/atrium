// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.getPosts` namespace.

/// A view of an actor's feed.
#[async_trait::async_trait]
pub trait GetPosts: crate::xrpc::XrpcClient {
    async fn get_posts(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "app.bsky.feed.getPosts",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub uris: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub posts: Vec<crate::app::bsky::feed::defs::PostView>,
}

pub enum Error {
}
