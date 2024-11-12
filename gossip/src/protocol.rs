use {
    crate::ping_pong::{Ping, Pong},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    PullRequest,
    PullResponse,
    PushMessage,
    PruneMessage,
    PingMessage(Ping),
    PongMessage(Pong),
}
