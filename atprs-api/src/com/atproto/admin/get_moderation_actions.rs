// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getModerationActions` namespace.

/// List moderation actions related to a subject.
pub trait GetModerationActions {
    fn get_moderation_actions(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub subject: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub actions: Vec<crate::com::atproto::admin::defs::ActionView>,
    pub cursor: Option<String>,
}

pub enum Error {
}