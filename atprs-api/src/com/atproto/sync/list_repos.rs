// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.listRepos` namespace.

/// List dids and root cids of hosted repos
#[async_trait::async_trait]
pub trait ListRepos: crate::xrpc::XrpcClient {
    async fn list_repos(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.sync.listRepos",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cursor: Option<String>,
    pub repos: Vec<Repo>,
}

pub enum Error {
}

// com.atproto.sync.listRepos#repo
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub did: String,
    pub head: String,
}
