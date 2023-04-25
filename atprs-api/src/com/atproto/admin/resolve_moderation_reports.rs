// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.resolveModerationReports` namespace.

/// Resolve moderation reports by an action.
#[async_trait::async_trait]
pub trait ResolveModerationReports: crate::xrpc::XrpcClient {
    async fn resolve_moderation_reports(&self, input: Input) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.admin.resolveModerationReports",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub action_id: i32,
    pub created_by: String,
    pub report_ids: Vec<i32>,
}

pub type Output = crate::com::atproto::admin::defs::ActionView;

pub enum Error {
}
