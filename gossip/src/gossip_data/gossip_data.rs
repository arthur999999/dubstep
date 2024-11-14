use {
    super::contact_info::ContactInfo,
    bv::BitVec,
    serde::{Deserialize, Serialize},
    solana_sdk::{clock::Slot, hash::Hash, pubkey::Pubkey, transaction::Transaction},
    std::collections::BTreeSet,
};

use super::legacy_contact_info::LegacyContactInfo;

pub enum GossipData {
    LegacyContactInfo(LegacyContactInfo),
    Vote(VoteIndex, Vote),
    LowestSlot(u8, LowestSlot),
    LegacySnapshotHashes(LegacySnapshotHashes),
    AccountsHashes(AccountsHashes),
    EpochSlots(EpochSlotsIndex, EpochSlots),
    LegacyVersion(LegacyVersion),
    Version(Version),
    NodeInstance(NodeInstance),
    DuplicateShred(DuplicateShredIndex, DuplicateShred),
    SnapshotHashes(SnapshotHashes),
    ContactInfo(ContactInfo),
    RestartLastVotedForkSlots(RestartLastVotedForkSlots),
    RestartHeaviestFork(RestartHeaviestFork),
}

type VoteIndex = u8;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub from: Pubkey,
    transaction: Transaction,
    pub wallclock: u64,
    slot: Option<Slot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LowestSlot {
    pub from: Pubkey,
    root: Slot,
    pub lowest: Slot,
    slots: BTreeSet<Slot>,
    stash: Vec<EpochIncompleteSlots>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EpochIncompleteSlots {
    first: Slot,
    compression: CompressionType,
    compressed_list: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CompressionType {
    Uncompressed,
    GZip,
    BZip2,
}

type LegacySnapshotHashes = AccountsHashes;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountsHashes {
    pub from: Pubkey,
    pub hashes: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

type EpochSlotsIndex = u8;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpochSlots {
    pub from: Pubkey,
    pub slots: Vec<CompressedSlots>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompressedSlots {
    Flate2(Flate2),
    Uncompressed(Uncompressed),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Flate2 {
    pub first_slot: Slot,
    pub num: usize,
    pub compressed: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Uncompressed {
    pub first_slot: Slot,
    pub num: usize,
    pub slots: BitVec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LegacyVersion {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: solana_version::LegacyVersion1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Version {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: solana_version::LegacyVersion2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInstance {
    from: Pubkey,
    wallclock: u64,
    timestamp: u64,
    token: u64,
}

pub type DuplicateShredIndex = u16;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DuplicateShred {
    pub from: Pubkey,
    pub wallclock: u64,
    pub slot: Slot,
    _unused: u32,
    _unused_shred_type: ShredType,
    num_chunks: u8,
    chunk_index: u8,
    chunk: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
enum ShredType {
    Data = 0b1010_0101,
    Code = 0b0101_1010,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotHashes {
    pub from: Pubkey,
    pub full: (Slot, Hash),
    pub incremental: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestartLastVotedForkSlots {
    pub from: Pubkey,
    pub wallclock: u64,
    offsets: SlotsOffsets,
    pub last_voted_slot: Slot,
    pub last_voted_hash: Hash,
    pub shred_version: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum SlotsOffsets {
    RunLengthEncoding(RunLengthEncoding),
    RawOffsets(RawOffsets),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RunLengthEncoding(Vec<u16>);

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RawOffsets(BitVec<u8>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestartHeaviestFork {
    pub from: Pubkey,
    pub wallclock: u64,
    pub last_slot: Slot,
    pub last_slot_hash: Hash,
    pub observed_stake: u64,
    pub shred_version: u16,
}
