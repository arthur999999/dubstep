use {
    crate::{
        gossip_data::{filter::DataFilter, gossip_data::GossipValue},
        ping_pong::{Ping, Pong},
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    PullRequest(DataFilter, GossipValue),
    PullResponse,
    PushMessage,
    PruneMessage,
    PingMessage(Ping),
    PongMessage(Pong),
}
