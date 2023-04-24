// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.getBlob` namespace.

/// Get a blob associated with a given repo.
pub trait GetBlob {
    fn get_blob(&self, input: Parameters) -> Result<(), Error>;
}

pub struct Parameters {
    /// The CID of the blob to fetch
    pub cid: String,
    /// The DID of the repo.
    pub did: String,
}


pub enum Error {
}