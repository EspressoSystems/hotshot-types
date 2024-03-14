//! Abstract storage type for storing DA proposals and VID shares
//!
//! This modules provides the [`BlockStorage`] trait.
//!

use snafu::Snafu;

use crate::{
    data::{DAProposal, VidDisperse},
    message::Proposal,
};

use super::node_implementation::NodeType;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum BlockStorageError {
    #[snafu(display("Failed to store the proposal due to an internal error"))]
    StoreError,

    #[snafu(display("Failed to retrieve the proposal due to an internal error"))]
    RetrieveError,
}

pub enum ProposalType<TYPES>
where
    TYPES: NodeType + 'static,
{
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>),
    VidDisperse(Proposal<TYPES, VidDisperse<TYPES>>),
}

/// Abstraction for storing the contents of DA proposals and VID shares.
pub trait BlockStorage<TYPES>: Clone + Send + Sync + Sized + 'static
where
    TYPES: NodeType + 'static,
{
    async fn append(&self, proposal: &ProposalType<TYPES>) -> Result<(), BlockStorageError>;
}
