extern crate device_query;

use device_query::{DeviceEvents, DeviceState, Keycode};

struct Bocks {
	state: DeviceState,
}

impl Bocks {
	fn new() -> Self {
		let state = DeviceState::new();

		Self { state }
	}

	fn start(&self) {
		let _guard = self.state.on_key_down(|key| {
			println!("down: {:#?}", key);
			// self.process(key);
		});

		let _guard = self.state.on_key_up(|key| {
			println!("up: {:#?}", key);
		});

		loop {}
	}

	fn process(&self, key: &Keycode) {
	}
}

fn main() {
	let bocks = Bocks::new();
	bocks.start();
}
