use {
    std::{net::SocketAddr, sync::Arc},
    tokio::{
        net::UdpSocket,
        sync::{
            mpsc::{self, Receiver, Sender},
            Mutex,
        },
        task,
    },
};

pub struct Connection {
    pub udp_socket: Arc<UdpSocket>,
    pub rx_in: Arc<Mutex<Receiver<(Vec<u8>, SocketAddr)>>>,
    pub tx_out: Sender<(Vec<u8>, SocketAddr)>,
    tx_in: Sender<(Vec<u8>, SocketAddr)>,
    rx_out: Arc<Mutex<Receiver<(Vec<u8>, SocketAddr)>>>,
}

impl Connection {
    pub async fn new(udp_socket: &str) -> tokio::io::Result<Arc<Self>> {
        let udp_socket = Arc::new(UdpSocket::bind(udp_socket).await?);
        let (tx_in, rx_in) = mpsc::channel(100);
        let (tx_out, rx_out) = mpsc::channel(100);

        let connection = Arc::new(Self {
            udp_socket,
            rx_in: Arc::new(Mutex::new(rx_in)),
            tx_out,
            tx_in,
            rx_out: Arc::new(Mutex::new(rx_out)),
        });

        Ok(connection)
    }

    pub fn start_receiving(self: &Arc<Self>) {
        let udp_socket = Arc::clone(&self.udp_socket);
        let tx_channel = self.tx_in.clone();

        task::spawn(async move {
            let mut buf = vec![0u8; 1260];
            loop {
                if let Ok((size, src)) = udp_socket.recv_from(&mut buf).await {
                    let msg = buf[..size].to_vec();
                    if tx_channel.send((msg, src)).await.is_err() {
                        eprintln!("Receiver dropped");
                        break;
                    }
                }
            }
        });
    }

    pub fn start_sending(self: &Arc<Self>) {
        let udp_socket = Arc::clone(&self.udp_socket);
        let rx_channel = Arc::clone(&self.rx_out);

        task::spawn(async move {
            let mut rx_channel = rx_channel.lock().await;
            while let Some((msg, addr)) = rx_channel.recv().await {
                if let Err(e) = udp_socket.send_to(&msg, &addr).await {
                    eprintln!("CONNECTION: Failed to send data to:{:?} {:?}", addr, e);
                }
            }
        });
    }
}
