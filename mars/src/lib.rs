use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;

use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

use skittles::SkittlesClient;

type Message = Vec<u8>;
type Tx = mpsc::UnboundedSender<(String, Message)>;

/// Pub/Sub
pub struct Mars(Arc<Mutex<HashMap<String, Vec<(Tx, SocketAddr)>>>>, SkittlesClient);

impl Mars {
    /// Constructor
    pub fn new(s: SkittlesClient) -> Self {
        Self {
            0: Arc::new(Mutex::new(HashMap::new())),
            1: s
        }
    }

    /// Add subscriber to given topics(comma separated)
    pub async fn add_subscriber(&mut self, s: TcpStream, t: &str) {
        let addr = match s.peer_addr() {
            Ok(s) => s,
            Err(_) => return,
        };
        
        let (tx, mut rx) = mpsc::unbounded_channel::<(String, Message)>();
        let topics = t.split(",");
        
        {
            let mut guard = self.0.lock().await;

            for topic in topics {
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

    /// Drop a subscriber from given topics
    pub async fn drop_subscriber(&mut self, t: &str, s: SocketAddr) {
        let topics = t.split(",");

        let mut guard = self.0.lock().await;

        for topic in topics {
            match guard.get_mut(topic) {
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
                _ => ()
            }
        }

        return;
    }

    /// Add new topics(comma separated)
    pub async fn add_topic(&mut self, s: &str) {
        let topics = s.split(",").map(|x| x.to_string());

        let mut guard = self.0.lock().await;
        for topic in topics {
            guard.insert(topic, vec![]);
        }
        self.1.log(format!("Added topics: {}", s).as_str()).await;
    }

    /// Drop topics(comma separated)
    pub async fn drop_topic(&mut self, s: &str) {
        let topics = s.split(",").map(|x| x.to_string());

        let mut guard = self.0.lock().await;
        for topic in topics {
            guard.remove(&topic);
        }

        self.1.log(format!("Dropped topics: {}", s).as_str()).await;
    }

    /// Send a message to everyone listening on a particular topic(single topic)
    pub async fn send_topic_message(&self, s: &str, msg: &[u8]) {
        match self.0.lock().await.get(s) {
            Some(vt) => {
                for (tx, _) in vt {
                    let _ = tx.send((s.to_string(), msg.to_vec()));
                }

                self.1.log(format!("Sent notification for topic: {}", s).as_str()).await;
            },
            None => ()
        }
    }

    /// Number of Topics present
    pub async fn size(&self) -> usize {
        self.0.lock().await.len()
    }

    /// Number of receivers for a topic
    pub async fn topic_size(&self, t: &str) -> usize {
        let mut cnt = 0;

        let topics = t.split(",");

        {
            let guard = self.0.lock().await;
            
            for topic in topics {
                match guard.get(topic) {
                    Some(v) => cnt += v.len(),
                    None => {}
                }
            }
        }
    
        cnt
    }
}