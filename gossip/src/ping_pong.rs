use {
    bincode::{serialize, Error},
    lru::LruCache,
    rand::Rng,
    serde::{Deserialize, Serialize},
    solana_sdk::{
        hash::{self, Hash},
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    std::{
        net::SocketAddr,
        num::NonZero,
        sync::Arc,
        time::{Duration, Instant},
    },
    thiserror::Error,
    tokio::sync::mpsc::Sender,
};

const GOSSIP_PING_TOKEN_SIZE: usize = 32;
const PING_PONG_HASH_PREFIX: &[u8] = "SOLANA_PING_PONG".as_bytes();

#[derive(Debug, Deserialize, Serialize)]
pub struct Pong {
    from: Pubkey,
    hash: Hash,
    signature: Signature,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ping {
    from: Pubkey,
    token: [u8; GOSSIP_PING_TOKEN_SIZE],
    signature: Signature,
}

impl Ping {
    fn new(token: [u8; GOSSIP_PING_TOKEN_SIZE], keypair: &Keypair) -> Result<Self, Error> {
        let signature = keypair.sign_message(&serialize(&token)?);
        let ping = Ping {
            from: keypair.pubkey(),
            token,
            signature,
        };
        Ok(ping)
    }

    pub fn rand(keypair: &Keypair) -> Result<Self, Error> {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.gen();

        Self::new(random_bytes, keypair)
    }

    pub async fn process(
        ping: Self,
        from: SocketAddr,
        tx_out: Sender<(Vec<u8>, SocketAddr)>,
        keypair: Arc<Keypair>,
    ) -> Result<(), PingPongErros> {
        let pong = match Pong::new(&ping, &keypair) {
            Ok(p) => p,
            Err(_) => return Err(PingPongErros::FailedToCreatePong),
        };

        pong.send(from, tx_out).await?;

        Ok(())
    }
}

impl Pong {
    pub fn new(ping: &Ping, keypair: &Keypair) -> Result<Self, Error> {
        let token = serialize(&ping.token)?;
        let hash = hash::hashv(&[PING_PONG_HASH_PREFIX, &token]);
        let pong = Pong {
            from: keypair.pubkey(),
            hash,
            signature: keypair.sign_message(hash.as_ref()),
        };
        Ok(pong)
    }

    pub fn from(&self) -> &Pubkey {
        &self.from
    }

    async fn send(
        &self,
        addr: SocketAddr,
        tx_out: Sender<(Vec<u8>, SocketAddr)>,
    ) -> Result<(), PingPongErros> {
        let message = match serialize(self) {
            Ok(m) => m,
            Err(_) => return Err(PingPongErros::FailedToSerealizePong),
        };

        match tx_out.send((message, addr)).await {
            Ok(_) => return Ok(()),
            Err(_) => return Err(PingPongErros::FailedToSendAPong),
        };
    }
}

#[derive(Debug, Error)]
pub enum PingPongErros {
    #[error("Failed to create pong to reply a ping")]
    FailedToCreatePong,
    #[error("Failed to serealize pong")]
    FailedToSerealizePong,
    #[error("Failed to send apong")]
    FailedToSendAPong,
}

pub struct PingCache {
    ttl: Duration,
    rate_limit_delay: Duration,
    pings: LruCache<(Pubkey, SocketAddr), Instant>,
    pongs: LruCache<(Pubkey, SocketAddr), Instant>,
    pending_cache: LruCache<Hash, (Pubkey, SocketAddr)>,
}

impl PingCache {
    pub fn new(ttl: Duration, rate_limit_delay: Duration, cap: NonZero<usize>) -> Self {
        Self {
            ttl,
            rate_limit_delay,
            pings: LruCache::new(cap),
            pongs: LruCache::new(cap),
            pending_cache: LruCache::new(cap),
        }
    }

    pub fn add(&mut self, pong: &Pong, socket: SocketAddr, now: Instant) -> bool {
        let node = (pong.from, socket);
        match self.pending_cache.peek(&pong.hash) {
            Some(value) if *value == node => {
                self.pings.pop(&node);
                self.pongs.put(node, now);
                self.pending_cache.pop(&pong.hash);
                true
            }
            _ => false,
        }
    }

    fn maybe_ping<F>(
        &mut self,
        now: Instant,
        node: (Pubkey, SocketAddr),
        mut pingf: F,
    ) -> Option<Ping>
    where
        F: FnMut() -> Option<Ping>,
    {
        match self.pings.peek(&node) {
            Some(t) if now.saturating_duration_since(*t) < self.rate_limit_delay => None,
            _ => {
                let ping = pingf()?;
                let token = serialize(&ping.token).ok()?;
                let hash = hash::hashv(&[PING_PONG_HASH_PREFIX, &token]);
                self.pending_cache.put(hash, node);
                self.pings.put(node, now);
                Some(ping)
            }
        }
    }

    pub fn check<F>(
        &mut self,
        now: Instant,
        node: (Pubkey, SocketAddr),
        pingf: F,
    ) -> (bool, Option<Ping>)
    where
        F: FnMut() -> Option<Ping>,
    {
        let (check, should_ping) = match self.pongs.get(&node) {
            None => (false, true),
            Some(t) => {
                let age = now.saturating_duration_since(*t);
                if age > self.ttl {
                    self.pongs.pop(&node);
                }
                (true, age > self.ttl / 8)
            }
        };
        let ping = if should_ping {
            self.maybe_ping(now, node, pingf)
        } else {
            None
        };
        (check, ping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pong() {
        let keypair = Keypair::new();
        let ping = Ping::rand(&keypair).expect("Failed to create ping");

        let expected_hash = hash::hashv(&[PING_PONG_HASH_PREFIX, &ping.token]);
        let pong = Pong::new(&ping, &keypair).expect("Failed to create pong");

        assert_eq!(
            pong.hash, expected_hash,
            "The pong hash does not match the expected hash"
        );
    }
}
