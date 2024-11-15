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
    thiserror::Error,
};

pub fn create_pull_request_message(
    contact_info: ContactInfo,
    filter: DataFilter,
    keypair: &Keypair,
) -> Result<Vec<u8>, PushMessagesErrors> {
    if contact_info.sockets().is_empty() {
        return Err(PushMessagesErrors::NoSocketEntry);
    }

    let signed_data = GossipValue::new_signed(GossipData::ContactInfo(contact_info), keypair);

    let protocol = Protocol::PullRequest(filter, signed_data);

    let message = match serialize(&protocol) {
        Ok(v) => v,
        Err(_) => return Err(PushMessagesErrors::SerializeFailed),
    };

    Ok(message)
}

#[derive(Debug, Error)]
pub enum PushMessagesErrors {
    #[error("No socket adress in contact info")]
    NoSocketEntry,
    #[error("Failed to serialize message")]
    SerializeFailed,
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

        let pull_request = create_pull_request_message(contact_info, filter, &keypair);

        assert!(pull_request.is_err())
    }

    #[test]
    fn test_create_pull_request() {
        let keypair = Keypair::new();
        let gossip: SocketAddr = "0.0.0.0:8100"
            .parse()
            .expect("Failed create entrypoint socket");
        let contact_info = ContactInfo::new(keypair.pubkey(), timestamp(), 0, gossip);
        let filter = DataFilter::default();

        let pull_request = create_pull_request_message(contact_info, filter, &keypair);

        assert!(pull_request.is_ok())
    }
}
