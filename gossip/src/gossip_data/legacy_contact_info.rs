use {
    serde::{Deserialize, Serialize},
    solana_sdk::pubkey::Pubkey,
    std::net::SocketAddr,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LegacyContactInfo {
    id: Pubkey,
    gossip: SocketAddr,
    tvu: SocketAddr,
    tvu_quic: SocketAddr,
    serve_repair_quic: SocketAddr,
    tpu: SocketAddr,
    tpu_forwards: SocketAddr,
    tpu_vote: SocketAddr,
    rpc: SocketAddr,
    rpc_pubsub: SocketAddr,
    serve_repair: SocketAddr,
    wallclock: u64,
    shred_version: u16,
}

impl LegacyContactInfo {
    pub fn pubkey(&self) -> &Pubkey {
        &self.id
    }
}
