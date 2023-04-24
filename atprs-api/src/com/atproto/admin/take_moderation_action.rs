// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.takeModerationAction` namespace.

/// Take a moderation action on a repo.
pub trait TakeModerationAction {
    fn take_moderation_action(&self, input: Input) -> Result<Output, Error>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub action: String,
    pub create_label_vals: Option<Vec<String>>,
    pub created_by: String,
    pub negate_label_vals: Option<Vec<String>>,
    pub reason: String,
    // pub subject: ...,
    pub subject_blob_cids: Option<Vec<String>>,
}

pub type Output = crate::com::atproto::admin::defs::ActionView;

pub enum Error {
    SubjectHasAction,
}