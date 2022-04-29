#[derive(Default)]
pub(crate) struct Rules {
	ram: f32,
	mem: f32,
	network: usize,
}

impl Rules {
	pub(crate) fn new(ram: f32, mem: f32, network: usize) -> Self {
		Self { ram, mem, network }
	}

	pub(crate) fn check_ram(&self, ram: f32) -> bool {
		self.ram < ram
	}

	pub(crate) fn check_mem(&self, mem: f32) -> bool {
		self.mem < mem
	}

	pub(crate) fn check_network(&self, network: usize) -> bool {
		self.network < network
	}

	pub(crate) fn check(&self, ram: f32, mem: f32, network: usize) -> (bool, bool, bool) {
		(self.check_ram(ram), self.check_mem(mem), self.check_network(network))
	}
}