// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getModerationActions` namespace.

/// List moderation actions related to a subject.
#[async_trait::async_trait]
pub trait GetModerationActions: crate::xrpc::XrpcClient {
    async fn get_moderation_actions(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.admin.getModerationActions",
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
    pub subject: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub actions: Vec<crate::com::atproto::admin::defs::ActionView>,
    pub cursor: Option<String>,
}

pub enum Error {
}
