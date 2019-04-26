use chrono::prelude::{DateTime, NaiveDateTime, Utc};
use primitives::crypto::signature::Signature;
use primitives::hash::CryptoHash;
use primitives::types::BlockIndex;

pub struct BlockHeader {
    /// Height of this block since the genesis block (height 0).
    pub height: BlockIndex,
    /// Hash of the block previous to this in the chain.
    pub prev_hash: CryptoHash,
    /// Root hash of the state at the previous block.
    pub prev_state_root: CryptoHash,
    /// Timestamp at which the block was built.
    pub timestamp: DateTime<Utc>,
    /// Authority signatures.
    pub signatures: Vec<Signature>,
    /// Total weight.
    pub total_weight: Weight,
}

impl BlockHeader {
    pub fn hash(&self) -> CryptoHash {
        CryptoHash::default()
    }
}

pub struct Bytes(Vec<u8>);

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Bytes>,
}

impl Block {
    pub fn hash(&self) -> CryptoHash {
        self.header.hash()
    }
}

pub enum BlockStatus {
    /// Block is the "next" block, updating the chain head.
    Next,
    /// Block does not update the chain head and is a fork.
    Fork,
    /// Block updates the chain head via a (potentially disruptive) "reorg".
    /// Previous block was not our previous chain head.
    Reorg,
}

/// Bridge between the chain and the rest of the system.
/// Handles downstream processing of valid blocks by the rest tof the system.
pub trait ChainAdapter {
    fn block_accepted(&self, block: &Block, status: BlockStatus);
}

pub struct NoopAdapter {}

impl ChainAdapter for NoopAdapter {
    fn block_accepted(&self, _block: &Block, _status: BlockStatus) {}
}

/// Bridge between the chain and the runtime.
/// Handles updating state given transactions.
pub trait RuntimeAdapter {}

/// The weight is defined as the number of unique authorities approving this fork.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Weight {
    num: u64,
}

/// The tip of a fork. A handle to the fork ancestry from its leaf in the
/// blockchain tree. References the max height and the latest and previous
/// blocks for convenience and the total weight.
#[derive(Debug, Clone, PartialEq)]
pub struct Tip {
    /// Height of the tip (max height of the fork)
    pub height: u64,
    /// Last block pushed to the fork
    pub last_block_hash: CryptoHash,
    /// Previous block
    pub prev_block_hash: CryptoHash,
    /// Total weight on that fork
    pub total_weight: Weight,
}

impl Tip {
    /// Creates a new tip based on provided header.
    pub fn from_header(header: &BlockHeader) -> Tip {
        Tip {
            height: header.height,
            last_block_hash: header.hash(),
            prev_block_hash: header.prev_hash,
            total_weight: header.total_weight,
        }
    }

    /// The hash of the underlying block.
    fn hash(&self) -> CryptoHash {
        self.last_block_hash
    }
}