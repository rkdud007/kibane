use celestia_tendermint::{
    block::{parts::Header, Commit},
    validator,
};

/// Tendermint block hash.  
pub type Hash = celestia_tendermint::hash::Hash;
/// Information about a tendermint validator.
pub type Validator = validator::Info;
/// A collection of the tendermint validators.
pub type ValidatorSet = validator::Set;

/// Namespace version size in bytes.
pub const NS_VER_SIZE: usize = 1;
/// Namespace id size in bytes.
pub const NS_ID_SIZE: usize = 28;
/// Namespace size in bytes.
pub const NS_SIZE: usize = NS_VER_SIZE + NS_ID_SIZE;
/// Namespaced hash.
pub type NamespacedHash = nmt_rs::NamespacedHash<NS_SIZE>;

#[derive(Debug)]
pub struct DataAvailabilityHeader {
    /// Merkle roots of the [`ExtendedDataSquare`] rows.
    ///
    /// [`ExtendedDataSquare`]: crate::rsmt2d::ExtendedDataSquare
    pub row_roots: Vec<NamespacedHash>,
    /// Merkle roots of the [`ExtendedDataSquare`] columns.
    ///
    /// [`ExtendedDataSquare`]: crate::rsmt2d::ExtendedDataSquare
    pub column_roots: Vec<NamespacedHash>,
}

#[derive(Debug)]
pub struct ExtendedHeader {
    /// Tendermint block header.
    pub header: Header,
    /// Commit metadata and signatures from validators committing the block.
    pub commit: Commit,
    /// Information about the set of validators commiting the block.
    pub validator_set: ValidatorSet,
    /// Header of the block data availability.
    pub dah: DataAvailabilityHeader,
}
