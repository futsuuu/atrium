// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.searchRepos` namespace.

/// Find repositories based on a search term.
#[async_trait::async_trait]
pub trait SearchRepos: crate::xrpc::XrpcClient {
    async fn search_repos(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.admin.searchRepos",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub repos: Vec<crate::com::atproto::admin::defs::RepoView>,
}

pub enum Error {
}
