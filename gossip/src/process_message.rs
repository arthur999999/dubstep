use {
    crate::{connection::Connection, ping_pong::Ping, protocol::Protocol},
    bincode::deserialize,
    solana_sdk::signature::Keypair,
    std::sync::Arc,
};

pub async fn process_message(connection: Arc<Connection>, keypair: Keypair) {
    let mut receive_channel = connection.rx_in.lock().await;
    let arc_keypair = Arc::new(keypair);

    while let Some((message, from)) = receive_channel.recv().await {
        let connection_clone = Arc::clone(&connection);
        let arc_keypair_clone = Arc::clone(&arc_keypair);
        tokio::spawn(async move {
            let protocol: Result<Protocol, _> = deserialize(&message);
            match protocol {
                Ok(protocol) => match protocol {
                    Protocol::PullRequest => (),
                    Protocol::PullResponse => (),
                    Protocol::PushMessage => (),
                    Protocol::PruneMessage => (),
                    Protocol::PingMessage(ping) => {
                        let _ = Ping::process(
                            ping,
                            from,
                            connection_clone.tx_out.clone(),
                            arc_keypair_clone,
                        )
                        .await;
                    }
                    Protocol::PongMessage(pong) => todo!(),
                },
                Err(_) => (),
            }
        });
    }
}
