use std::net::SocketAddr;
use std::collections::{HashMap};

use tokio::net::{TcpStream};
use tokio::sync::mpsc;

type Message = Vec<u8>;
type Tx = mpsc::UnboundedSender<(String, Message)>;

pub struct Mars(HashMap<String, Vec<(Tx, SocketAddr)>>);

impl Mars {
    pub fn new() -> Self {
        Self {
            0: HashMap::new()
        }
    }

    pub fn add_subscriber(&mut self, s: TcpStream, t: &str) {
        let addr = match s.peer_addr() {
            Ok(s) => s,
            Err(_) => return,
        };
        
        if !self.0.contains_key(t) {
            return;
        }

        let (tx, mut rx) = mpsc::unbounded_channel::<(String, Message)>();

        let vt = match self.0.get_mut(t) {
            Some(vt) => vt,
            None => return,
        };

        vt.push((tx, addr));

        tokio::spawn(async move {
            loop {
                if let Some(msg) = rx.recv().await {
                    let _ = s.writable().await;
                    let _ = s.try_write(msg.1.as_ref());
                } else {
                    break;
                }
            }
        });
    }

    pub fn drop_subscriber(&mut self, t: &str, s: SocketAddr) {
        match self.0.get_mut(t) {
            Some(vt) => {
                let mut i = 0;

                for e in vt.iter() {
                    if e.1 == s {
                        break;
                    }
                    i += 1;
                }

                vt.remove(i);
            },
            _ => (),
        }

        return;
    }

    pub fn add_topic(&mut self, s: &str) {
        self.0.insert(s.to_string(), vec![]);
    }

    pub fn drop_topic(&mut self, s: &str) {
        self.0.remove(s);
    }

    pub fn send_topic_message(&self, s: &str, msg: &[u8]) {
        match self.0.get(s) {
            Some(vt) => {
                for (tx, _) in vt {
                    let _ = tx.send((s.to_string(), msg.to_vec()));
                }
            },
            None => ()
        }
    }
}