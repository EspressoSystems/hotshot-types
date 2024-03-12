//! Error type for `HotShot`
//!
//! This module provides [`HotShotError`], which is an enum representing possible faults that can
//! occur while interacting with this crate.

use crate::traits::{
    block_contents::BlockPayload, node_implementation::NodeType, storage::StorageError,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::num::NonZeroU64;

#[cfg(async_executor_impl = "async-std")]
use async_std::future::TimeoutError;
#[cfg(async_executor_impl = "tokio")]
use tokio::time::error::Elapsed as TimeoutError;
#[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

/// Error type for `HotShot`
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum HotShotError<TYPES: NodeType> {
    /// Failed to Message the leader in the given stage
    #[snafu(display("Failed to message leader with error: {source}"))]
    FailedToMessageLeader {
        /// The underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Failed to broadcast a message on the network
    #[snafu(display("Failed to broadcast a message"))]
    FailedToBroadcast {
        /// The underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Failure in the block.
    #[snafu(display("Failed to build or verify a block: {source}"))]
    BlockError {
        /// The underlying block error.
        source: <TYPES::BlockPayload as BlockPayload>::Error,
    },
    /// Failure in networking layer
    #[snafu(display("Failure in networking layer: {source}"))]
    NetworkFault {
        /// Underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Item was not present in storage
    LeafNotFound {/* TODO we should create a way to to_string */},
    /// Error accesing storage
    StorageError {
        /// Underlying error
        source: StorageError,
    },
    /// Invalid state machine state
    #[snafu(display("Invalid state machine state: {}", context))]
    InvalidState {
        /// Context
        context: String,
    },
    /// HotShot timed out waiting for msgs
    TimeoutError {
        /// source of error
        source: TimeoutError,
    },
    /// HotShot timed out during round
    ViewTimeoutError {
        /// view number
        view_number: TYPES::Time,
        /// The state that the round was in when it timed out
        state: RoundTimedoutState,
    },
    /// Not enough valid signatures for a quorum
    #[snafu(display("Insufficient number of valid signatures: the threshold is {}, but only {} signatures were valid", threshold, num_valid_signatures))]
    InsufficientValidSignatures {
        /// Number of valid signatures
        num_valid_signatures: usize,
        /// Threshold of signatures needed for a quorum
        threshold: NonZeroU64,
    },
    /// Miscelaneous error
    /// TODO fix this with
    /// #181 <https://github.com/EspressoSystems/HotShot/issues/181>
    Misc {
        /// source of error
        context: String,
    },
    /// Internal value used to drive the state machine
    Continue,
}

// Implement Serialize for HotShotError
impl<TYPES: NodeType> Serialize for HotShotError<TYPES> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HotShotError::FailedToMessageLeader { source } => {
                HotShotError::<TYPES>::FailedToMessageLeader { source: *source }
                    .serialize(serializer)
            }
            HotShotError::FailedToBroadcast { source } => {
                HotShotError::<TYPES>::FailedToBroadcast { source: *source }.serialize(serializer)
            }
            HotShotError::BlockError { source } => {
                HotShotError::<TYPES>::BlockError { source: *source }.serialize(serializer)
            }
            HotShotError::NetworkFault { source } => {
                HotShotError::<TYPES>::NetworkFault { source: *source }.serialize(serializer)
            }
            HotShotError::LeafNotFound {} => {
                HotShotError::<TYPES>::LeafNotFound {}.serialize(serializer)
            }
            HotShotError::StorageError { source } => {
                HotShotError::<TYPES>::StorageError { source: *source }.serialize(serializer)
            }
            HotShotError::InvalidState { context } => {
                HotShotError::<TYPES>::InvalidState { context: *context }.serialize(serializer)
            }
            HotShotError::TimeoutError { source } => {
                HotShotError::<TYPES>::TimeoutError { source: *source }.serialize(serializer)
            }
            HotShotError::ViewTimeoutError { view_number, state } => {
                HotShotError::<TYPES>::ViewTimeoutError {
                    view_number: *view_number,
                    state: *state,
                }
                .serialize(serializer)
            }
            HotShotError::InsufficientValidSignatures {
                num_valid_signatures,
                threshold,
            } => HotShotError::<TYPES>::InsufficientValidSignatures {
                num_valid_signatures: *num_valid_signatures,
                threshold: *threshold,
            }
            .serialize(serializer),
            HotShotError::Misc { context } => {
                HotShotError::<TYPES>::Misc { context: *context }.serialize(serializer)
            }
            HotShotError::Continue => HotShotError::<TYPES>::Continue.serialize(serializer),
        }
    }
}

// Implement Deserialize for HotShotError
impl<'de, TYPES: NodeType> Deserialize<'de> for HotShotError<TYPES>
where
    TYPES: NodeType,
    <TYPES::BlockPayload as BlockPayload>::Error: Deserialize<'de>,
    crate::traits::network::NetworkError: Deserialize<'de>,
    RoundTimedoutState: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hotshot_error_enum: HotShotError<TYPES> = HotShotError::deserialize(deserializer)?;
        match hotshot_error_enum {
            HotShotError::FailedToMessageLeader { source } => {
                Ok(HotShotError::FailedToMessageLeader { source })
            }
            HotShotError::FailedToBroadcast { source } => {
                Ok(HotShotError::FailedToBroadcast { source })
            }
            HotShotError::BlockError { source } => Ok(HotShotError::BlockError { source }),
            HotShotError::NetworkFault { source } => Ok(HotShotError::NetworkFault { source }),
            HotShotError::LeafNotFound {} => Ok(HotShotError::LeafNotFound {}),
            HotShotError::StorageError { source } => Ok(HotShotError::StorageError { source }),
            HotShotError::InvalidState { context } => Ok(HotShotError::InvalidState { context }),
            HotShotError::TimeoutError { source } => Ok(HotShotError::TimeoutError { source }),
            HotShotError::ViewTimeoutError { view_number, state } => {
                Ok(HotShotError::ViewTimeoutError { view_number, state })
            }
            HotShotError::InsufficientValidSignatures {
                num_valid_signatures,
                threshold,
            } => Ok(HotShotError::InsufficientValidSignatures {
                num_valid_signatures,
                threshold,
            }),
            HotShotError::Misc { context } => Ok(HotShotError::Misc { context }),
            HotShotError::Continue => Ok(HotShotError::Continue),
        }
    }
}

/// Contains information about what the state of the hotshot-consensus was when a round timed out
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RoundTimedoutState {
    /// Leader is in a Prepare phase and is waiting for a HighQC
    LeaderWaitingForHighQC,
    /// Leader is in a Prepare phase and timed out before the round min time is reached
    LeaderMinRoundTimeNotReached,
    /// Leader is waiting for prepare votes
    LeaderWaitingForPrepareVotes,
    /// Leader is waiting for precommit votes
    LeaderWaitingForPreCommitVotes,
    /// Leader is waiting for commit votes
    LeaderWaitingForCommitVotes,

    /// Replica is waiting for a prepare message
    ReplicaWaitingForPrepare,
    /// Replica is waiting for a pre-commit message
    ReplicaWaitingForPreCommit,
    /// Replica is waiting for a commit message
    ReplicaWaitingForCommit,
    /// Replica is waiting for a decide message
    ReplicaWaitingForDecide,

    /// HotShot-testing tried to collect round events, but it timed out
    TestCollectRoundEventsTimedOut,
}
