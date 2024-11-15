use {
    super::{contact_info::ContactInfo, legacy_contact_info::LegacyContactInfo},
    bincode::serialize,
    bv::BitVec,
    serde::{Deserialize, Serialize},
    solana_sdk::{
        clock::Slot,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signable, Signature},
        transaction::Transaction,
    },
    std::{
        borrow::{Borrow, Cow},
        collections::BTreeSet,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GossipValue {
    pub signature: Signature,
    pub data: GossipData,
}

impl GossipValue {
    fn new_unsigned(data: GossipData) -> Self {
        Self {
            signature: Signature::default(),
            data,
        }
    }

    pub fn new_signed(data: GossipData, keypair: &Keypair) -> Self {
        let mut value = Self::new_unsigned(data);
        value.sign(keypair);
        value
    }

    pub fn pubkey(&self) -> Pubkey {
        match &self.data {
            GossipData::LegacyContactInfo(contact_info) => *contact_info.pubkey(),
            GossipData::Vote(_, vote) => vote.from,
            GossipData::LowestSlot(_, slots) => slots.from,
            GossipData::LegacySnapshotHashes(hash) => hash.from,
            GossipData::AccountsHashes(hash) => hash.from,
            GossipData::EpochSlots(_, p) => p.from,
            GossipData::LegacyVersion(version) => version.from,
            GossipData::Version(version) => version.from,
            GossipData::NodeInstance(node) => node.from,
            GossipData::DuplicateShred(_, shred) => shred.from,
            GossipData::SnapshotHashes(hash) => hash.from,
            GossipData::ContactInfo(node) => *node.pubkey(),
            GossipData::RestartLastVotedForkSlots(slots) => slots.from,
            GossipData::RestartHeaviestFork(fork) => fork.from,
        }
    }
}

impl Signable for GossipValue {
    fn pubkey(&self) -> Pubkey {
        self.pubkey()
    }

    fn signable_data(&self) -> Cow<[u8]> {
        Cow::Owned(serialize(&self.data).expect("failed to serialize CrdsData"))
    }

    fn get_signature(&self) -> Signature {
        self.signature
    }

    fn set_signature(&mut self, signature: Signature) {
        self.signature = signature
    }

    fn verify(&self) -> bool {
        self.get_signature()
            .verify(self.pubkey().as_ref(), self.signable_data().borrow())
    }
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
