use std::{sync::mpsc::channel, thread};

use rdev::{listen, Event};
use uinput::{event::{Controller, controller}, Device};

struct Mapping {
	start: String,
}

struct Rekt {
	device: Device,
	mapping: Mapping,
}

impl Rekt {
	fn new() -> Self {
		let device = uinput::default().unwrap()
			.name("rekt").unwrap()
			.event(Controller::All).unwrap()
			.create().unwrap()
		;

		Self {
			device,
			mapping: Mapping {
				start: "f".to_string(),
			},
		}
	}

	fn process(&mut self, event: Event) {
		println!("process: {:?}", event);

		match event.name {
			Some(string) => {
				if string == self.mapping.start {
					self.device.press(&controller::GamePad::Start).unwrap();
				}
			},
			None => (),
		}

		self.update();
	}

	fn update(&mut self) {
		self.device.synchronize().unwrap();
	}
}

fn main() {
	let mut rekt = Rekt::new();
	let (send_chan, recv_chan) = channel();

	let _listener = thread::spawn(move || {
		listen(move |event| {
			send_chan
				.send(event)
				.unwrap_or_else(|e| println!("Could not send event: {:?}", e));
		})
			.expect("Could not listen")
		;
	});

	for event in recv_chan.iter() {
		rekt.process(event);
	}

	println!("starting...");
}
