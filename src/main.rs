use std::{sync::mpsc::channel, thread};

use rdev::{listen, Event, EventType, Key};
use uinput::{event::{Controller, controller::{self, GamePad}, absolute::{self, Wheel}, Absolute, self}, Device};

struct Mapping {
	// face
	start: Key,
	a: Key,
	b: Key,
	x: Key,
	y: Key,
	z: Key,

	// triggers
	l: Key,
	r: Key,
	ms: Key,
	ls: Key,

	// stick
	up: Key,
	down: Key,
	left: Key,
	right: Key,

	// c-stick
	c_up: Key,
	c_down: Key,
	c_left: Key,
	c_right: Key,

	// // dpad
	// d_up: Key,
	// d_down: Key,
	// d_left: Key,
	// d_right: Key,

	// mods
	mod_x: Key,
	mod_y: Key,
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
			.event(Absolute::Wheel(Wheel::Rudder)).unwrap()
				.min(-128)
				.max(128)
			.event(Absolute::Wheel(Wheel::Gas)).unwrap()
				.min(-128)
				.max(128)
			.create().unwrap()
		;

		Self {
			device,
			mapping: Mapping {
				// face
				start: Key::KeyF,
				a: Key::Kp0,
				b: Key::Kp4,
				x: Key::Kp8,
				y: Key::KpDivide,
				z: Key::Kp9,

				// triggers
				l: Key::KeyA,
				r: Key::Kp7,
				ms: Key::KpReturn,
				ls: Key::KpPlus,

				// stick
				up: Key::Dot,
				down: Key::KeyE,
				left: Key::KeyO,
				right: Key::KeyU,

				// c-stick
				c_up: Key::UpArrow,
				c_down: Key::DownArrow,
				c_left: Key::LeftArrow,
				c_right: Key::RightArrow,

				// // dpad
				// d_up: "",
				// d_down: "",
				// d_left: "",
				// d_right: "",

				// mods
				mod_x: Key::KeyK,
				mod_y: Key::Space,
			},
		}
	}

	fn process(&mut self, event: Event) {
		match event.event_type {
			EventType::KeyPress(key) => {
				self.press(key);
			},
			EventType::KeyRelease(key) => {
				self.release(key);
			},
			_ => (),
		}

		self.update();
	}

	fn press(&mut self, key: Key) {
		match key {
			// face
			k if k == self.mapping.start => {
				self.device.press(&GamePad::Start).unwrap();
			},
			k if k == self.mapping.a => {
				self.device.press(&GamePad::A).unwrap();
			},
			k if k == self.mapping.b => {
				self.device.press(&GamePad::B).unwrap();
			},
			k if k == self.mapping.x => {
				self.device.press(&GamePad::X).unwrap();
			},
			k if k == self.mapping.y => {
				self.device.press(&GamePad::Y).unwrap();
			},
			k if k == self.mapping.z => {
				self.device.press(&GamePad::ThumbR).unwrap();
			},

			// triggers
			k if k == self.mapping.l => {
				self.device.press(&GamePad::TL).unwrap();
				self.device.position(&Wheel::Rudder, 128).unwrap();
			},
			k if k == self.mapping.ms => {
				self.device.position(&Wheel::Rudder, 50).unwrap();
			},
			k if k == self.mapping.ls => {
				self.device.position(&Wheel::Rudder, 22).unwrap();
			},
			k if k == self.mapping.r => {
				self.device.press(&GamePad::TR).unwrap();
				self.device.position(&Wheel::Gas, 128).unwrap();
			},

			_ => println!("pressed: {:?}", key),
		}
	}

	fn release(&mut self, key: Key) {
		match key {
			// face
			k if k == self.mapping.start => {
				self.device.release(&GamePad::Start).unwrap();
			},
			k if k == self.mapping.a => {
				self.device.release(&GamePad::A).unwrap();
			},
			k if k == self.mapping.b => {
				self.device.release(&GamePad::B).unwrap();
			},
			k if k == self.mapping.x => {
				self.device.release(&GamePad::X).unwrap();
			},
			k if k == self.mapping.y => {
				self.device.release(&GamePad::Y).unwrap();
			},
			k if k == self.mapping.z => {
				self.device.release(&GamePad::ThumbR).unwrap();
			},

			// triggers
			k if k == self.mapping.l => {
				self.device.release(&GamePad::TL).unwrap();
				self.device.position(&Wheel::Rudder, 0).unwrap();
			},
			k if k == self.mapping.ms => {
				self.device.position(&Wheel::Rudder, 0).unwrap();
			},
			k if k == self.mapping.ls => {
				self.device.position(&Wheel::Rudder, 0).unwrap();
			},
			k if k == self.mapping.r => {
				self.device.release(&GamePad::TR).unwrap();
				self.device.position(&Wheel::Gas, 0).unwrap();
			},

			_ => (),
		}
	}

	fn update(&mut self) {
		self.device.synchronize().unwrap();
	}
}

fn main() {
	let mut rekt = Rekt::new();
	let (send_chan, recv_chan) = channel();

	println!("starting...");

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
}
