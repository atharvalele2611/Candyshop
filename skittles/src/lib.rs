use hyper::{Request, Body, Method, Client};

#[derive(Clone)]
pub struct SkittlesClient {
	ip: String,
	name: String,
	log: bool
}

impl SkittlesClient {
	pub fn new(name: String, ip: String, log: bool) -> Self {
		SkittlesClient { ip: format!("http://{}/log", ip.to_string()), name: format!("({}) ", name), log: log }
	}

	pub async fn log(&self, msg: &str) {
		if self.log {
			let mut l = self.name.clone();
			l.push_str(msg);
			let req = Request::builder()
				.method(Method::POST)
				.uri(&self.ip)
				.header("content-type", "application/text")
				.body(Body::from(l)).unwrap();

			let client = Client::new();

			// We have a laidback logger. We do not care if this log went through or no.
			let _ = client.request(req).await;	
		}
	}
}