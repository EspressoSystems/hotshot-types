//! Common traits for the `HotShot` protocol
pub mod block_contents;
pub mod block_storage;
pub mod consensus_api;
pub mod election;
pub mod metrics;
pub mod network;
pub mod node_implementation;
pub mod qc;
pub mod signature_key;
pub mod stake_table;
pub mod states;

pub use block_contents::BlockPayload;
pub use states::ValidatedState;
