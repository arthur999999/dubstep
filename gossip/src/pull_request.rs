use {
    crate::{
        gossip_data::{
            contact_info::ContactInfo,
            filter::DataFilter,
            gossip_data::{GossipData, GossipValue},
        },
        protocol::Protocol,
    },
    bincode::serialize,
    solana_sdk::signature::Keypair,
    std::net::SocketAddr,
    thiserror::Error,
    tokio::sync::mpsc::Sender,
};

pub struct PullRequest {
    pub message: Vec<u8>,
    addr: SocketAddr,
}

impl PullRequest {
    pub fn new(
        contact_info: ContactInfo,
        filter: DataFilter,
        keypair: &Keypair,
        addr: SocketAddr,
    ) -> Result<PullRequest, PushMessagesErrors> {
        if contact_info.sockets().is_empty() {
            return Err(PushMessagesErrors::NoSocketEntry);
        }

        let signed_data = GossipValue::new_signed(GossipData::ContactInfo(contact_info), keypair);

        let protocol = Protocol::PullRequest(filter, signed_data);

        let message = match serialize(&protocol) {
            Ok(v) => v,
            Err(_) => return Err(PushMessagesErrors::SerializeFailed),
        };

        Ok(Self { message, addr })
    }

    pub async fn send(
        &self,
        tx_out: Sender<(Vec<u8>, SocketAddr)>,
    ) -> Result<(), PushMessagesErrors> {
        match tx_out.send((self.message.clone(), self.addr)).await {
            Ok(_) => return Ok(()),
            Err(_) => return Err(PushMessagesErrors::FailedToSendPullRequest),
        };
    }
}

#[derive(Debug, Error)]
pub enum PushMessagesErrors {
    #[error("No socket adress in contact info")]
    NoSocketEntry,
    #[error("Failed to serialize message")]
    SerializeFailed,
    #[error("Failed to send pull request message")]
    FailedToSendPullRequest,
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_sdk::{signer::Signer, timing::timestamp},
        std::net::SocketAddr,
    };

    #[test]
    fn test_create_pull_request_with_no_gossip_addres() {
        let keypair = Keypair::new();
        let contact_info = ContactInfo::default();
        let filter = DataFilter::default();
        let addr: SocketAddr = "0.0.0.0:0".parse().expect("Failed create addr socket");

        let pull_request = PullRequest::new(contact_info, filter, &keypair, addr);

        assert!(pull_request.is_err())
    }

    #[test]
    fn test_create_pull_request() {
        let keypair = Keypair::new();
        let gossip: SocketAddr = "0.0.0.0:0".parse().expect("Failed create gossip socket");
        let addr: SocketAddr = "0.0.0.0:0".parse().expect("Failed create addr socket");
        let contact_info = ContactInfo::new(keypair.pubkey(), timestamp(), 0, gossip);
        let filter = DataFilter::default();

        let pull_request = PullRequest::new(contact_info, filter, &keypair, addr);

        assert!(pull_request.is_ok())
    }
}
