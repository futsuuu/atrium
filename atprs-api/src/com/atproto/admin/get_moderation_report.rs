// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getModerationReport` namespace.

/// View details about a moderation report.
pub trait GetModerationReport {
    fn get_moderation_report(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub id: i32,
}

pub type Output = crate::com::atproto::admin::defs::ReportViewDetail;

pub enum Error {
}