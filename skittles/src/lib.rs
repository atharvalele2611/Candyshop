use tokio::sync::Mutex;
use std::sync::Arc;
use hyper::{Request, Body, Method, Client};

struct ClientInner {
	ip: String,
	name: String,
	log: bool
}

pub struct SkittlesClient {
	c: Arc<Mutex<ClientInner>>
}

impl SkittlesClient {
	pub fn new(name: String, ip: String, log: bool) -> Self {
		SkittlesClient {
			c: Arc::new(Mutex::new(ClientInner {
				ip: format!("http://{}/log", ip.to_string()),
				name: format!("({}) ", name),
				log: log
			}))
		}
	}

	pub async fn log(&self, msg: &str) {
		let c = self.c.lock().await;

		if c.log {
			let mut l = c.name.clone();
			l.push_str(msg);
			let ip = c.ip.clone();
			let req = Request::builder()
				.method(Method::POST)
				.uri(ip)
				.header("content-type", "application/text")
				.body(Body::from(l)).unwrap();

			let client = Client::new();

			// We have a laidback logger. We do not care if this log went through or no.
			let _ = client.request(req).await;	
		}
	}
}

impl Clone for SkittlesClient {
	fn clone(&self) -> Self {
		SkittlesClient { c: Arc::clone(&self.c) }
	}
}