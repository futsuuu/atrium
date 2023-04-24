// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.server.createInviteCode` namespace.

/// Create an invite code.
pub trait CreateInviteCode {
    fn create_invite_code(&self, input: Input) -> Result<Output, Error>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub for_account: Option<String>,
    pub use_count: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub code: String,
}

pub enum Error {
}