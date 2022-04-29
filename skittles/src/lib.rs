use hyper::{Request, Body, Method, Client};

pub struct SkittlesClient {
	ip: String
}

impl SkittlesClient {
	pub fn new(ip: String) -> Self {
		SkittlesClient { ip: ip.to_string() }
	}

	pub async fn log(&self, msg: String) {
		let req = Request::builder()
			.method(Method::POST)
			.uri(&self.ip)
			.header("content-type", "application/text")
			.body(Body::from(msg)).unwrap();

		let client = Client::new();

		// We have a laidback logger. We do not care if this log went through or no.
		let _ = client.request(req).await;
	}
}