// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getInviteCodes` namespace.

/// Admin view of invite codes
pub trait GetInviteCodes {
    fn get_invite_codes(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub sort: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub codes: Vec<crate::com::atproto::server::defs::InviteCode>,
    pub cursor: Option<String>,
}

pub enum Error {
}