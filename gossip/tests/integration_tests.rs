use {
    bincode::{deserialize, serialize},
    dotenv::dotenv,
    dubstep_gossip::{connection::Connection, ping_pong::Ping, protocol::Protocol},
    solana_sdk::signature::Keypair,
    std::{env, net::SocketAddr, time::Duration},
    tokio::time::timeout,
};

#[tokio::test]
async fn test_send_ping() {
    //if you are using the mainnet or testnet maybe the test will fail
    //only because the entry point dont respond,
    //if the test fail you should try again some times to ensure the test really is working or just use the devnet
    let (udp, entry_point, _, _) = get_env_vars();
    let connection = Connection::new(&udp)
        .await
        .expect("Failed to create connection");

    let solana_entrypoint: SocketAddr = entry_point
        .parse()
        .expect("Failed create entrypoint socket");

    connection.start_sending();
    connection.start_receiving();

    let keypair = Keypair::new();
    let ping = Ping::rand(&keypair).expect("Failed to create ping");

    let message = serialize(&Protocol::PingMessage(ping)).expect("Failed to serealize ping");

    if let Err(e) = connection.tx_out.send((message, solana_entrypoint)).await {
        panic!("Failed to send message: {:?}", e);
    }

    let result = timeout(Duration::from_secs(10), async {
        loop {
            let msg_opt = {
                let mut listen_channel = connection.rx_in.lock().await;
                listen_channel.recv().await
            };
            if let Some((msg, from)) = msg_opt {
                if from == solana_entrypoint {
                    let protocol: Protocol =
                        deserialize(&msg).expect("Failed to deserialize message");
                    match protocol {
                        Protocol::PongMessage(_pong) => {
                            return Ok(());
                        }
                        _ => {
                            return Err("Received a message that is not a Pong");
                        }
                    }
                }
            }
        }
    })
    .await;

    assert!(result.is_ok(), "{}", result.unwrap_err());
}

fn get_env_vars() -> (String, String, String, String) {
    dotenv().ok();

    let udp_socket =
        env::var("UDP_SOCKET_TEST").expect("Failed to get UDP_SOCKET_TEST in .env file");

    let enrty_point =
        env::var("ENTRY_POINT_TEST").expect("Failed to get ENTRY_POINT_TEST in .env file");

    let gossip_socket_addr = env::var("GOSSIP_SOCKET_ADDR_TEST")
        .expect("Failed to get GOSSIP_SOCKET_ADDR_TEST in .env file");

    let peer_node = env::var("PEER_NODE_TEST").expect("Failed to get PEER_NODE_TEST in .env file");

    (udp_socket, enrty_point, gossip_socket_addr, peer_node)
}
