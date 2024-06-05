//! Error struct of project dynbinseq

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// Sequence is larger than what node can store.
    #[error("Sequence is larger than what a Node can store")]
    SeqToLargeForNode,

    /// Index larger than sequence length
    #[error("Index larger than sequence length")]
    IndexLargerThanSeq,
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
