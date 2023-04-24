// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getModerationReports` namespace.

/// List moderation reports related to a subject.
pub trait GetModerationReports {
    fn get_moderation_reports(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub resolved: Option<bool>,
    pub subject: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub cursor: Option<String>,
    pub reports: Vec<crate::com::atproto::admin::defs::ReportView>,
}

pub enum Error {
}