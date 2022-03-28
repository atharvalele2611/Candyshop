use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;

use tokio::net::{TcpStream};
use tokio::sync::{mpsc, Mutex};

type Message = Vec<u8>;
type Tx = mpsc::UnboundedSender<(String, Message)>; // per stream

pub struct Mars(Arc<Mutex<HashMap<String, Vec<(Tx, SocketAddr)>>>>);

impl Mars {
    pub fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn add_subscriber(&mut self, s: TcpStream, t: &str) {
        let addr = match s.peer_addr() {
            Ok(s) => s,
            Err(_) => return,
        };
        
        let (tx, mut rx) = mpsc::unbounded_channel::<(String, Message)>();
    
        {
            let mut guard = self.0.lock().await;

            for topic in t.split(",") {
                match guard.get_mut(topic) {
                    Some(v) => {
                        let tn = tx.clone();
                        v.push((tn, addr));
                    },
                    None => {}
                }
            }
        }

        let _ = tokio::spawn(async move {
            loop {
                if let Some(msg) = rx.recv().await {
                    let _ = s.writable().await;
                    let _ = s.try_write(msg.1.as_ref());
                } else {
                    break;
                }
            }
        }).await;

        self.drop_subscriber(t, addr).await;
    }

    pub async fn drop_subscriber(&mut self, t: &str, s: SocketAddr) {
        match self.0.lock().await.get_mut(t) {
            Some(vt) => {
                let mut i = 0;

                for e in vt.iter() {
                    if e.1 == s {
                        break;
                    }
                    i += 1;
                }

                vt.swap_remove(i);
            },
            _ => (),
        }

        return;
    }

    pub async fn add_topic(&mut self, s: &str) {
        self.0.lock().await.insert(s.to_string(), vec![]);
    }

    pub async fn drop_topic(&mut self, s: &str) {
        self.0.lock().await.remove(s);
    }

    pub async fn send_topic_message(&self, s: &str, msg: &[u8]) {
        match self.0.lock().await.get(s) {
            Some(vt) => {
                for (tx, _) in vt {
                    let _ = tx.send((s.to_string(), msg.to_vec()));
                }
            },
            None => ()
        }
    }
}