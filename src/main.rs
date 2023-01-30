use std::{sync::mpsc::channel, thread};

use rdev::{listen, EventType, Key};
use uinput::{event::{Controller, controller::GamePad, absolute::{self, Wheel, Position}, Absolute}, Device};

mod controller;

struct Rekt {
	device: Device,
	input: controller::InputMap,
	output: controller::OutputMap,
	state: controller::State,
}

impl Rekt {
	fn new() -> Self {
		let device = uinput::default().unwrap()
			.name("rekt").unwrap()
			.event(Controller::All).unwrap()
			.event(Absolute::Wheel(absolute::Wheel::Rudder)).unwrap()
				.min(-128)
				.max(128)
			.event(Absolute::Wheel(absolute::Wheel::Throttle)).unwrap()
				.min(-128)
				.max(128)
			.event(Absolute::Position(absolute::Position::X)).unwrap()
				.min(0)
				.max(255)
			.event(Absolute::Position(absolute::Position::Y)).unwrap()
				.min(0)
				.max(255)
			.event(Absolute::Position(absolute::Position::RX)).unwrap()
				.min(0)
				.max(255)
			.event(Absolute::Position(absolute::Position::RY)).unwrap()
				.min(0)
				.max(255)
			.create().unwrap()
		;

		Self {
			device,

			input: controller::InputMap {
				// face
				start: Key::KeyF,
				a: Key::Kp0,
				b: Key::Kp4,
				x: Key::Kp5,
				y: Key::Kp8,
				z: Key::Kp6,

				// triggers
				l: Key::KeyA,
				r: Key::Kp7,
				lm: Key::KpPlus,
				ls: Key::KpReturn,

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

				// mods
				mod_x: Key::Space,
				mod_y: Key::Alt,

				// debug
				debug: Key::SemiColon,
			},

			output: controller::OutputMap {
				// face
				start: GamePad::Start,
				a: GamePad::A,
				b: GamePad::B,
				x: GamePad::X,
				y: GamePad::Y,
				z: GamePad::ThumbR,

				// triggers
				l: GamePad::TL,
				r: GamePad::TR,
				la: Wheel::Rudder,
				ra: Wheel::Throttle,

				// stick
				horizontal: Position::X,
				vertical: Position::Y,

				// c-stick
				c_horizontal: Position::RX,
				c_vertical: Position::RY,

				// dpad
				d_up: GamePad::North,
				d_down: GamePad::North,
				d_left: GamePad::North,
				d_right: GamePad::North,
			},

			state: controller::State::new(),
		}
	}

	fn handle(&mut self, event: rdev::Event) {
		match event.event_type {
			EventType::KeyPress(key) => self.press(key),
			EventType::KeyRelease(key) => self.release(key),
			_ => (),
		}
	}

	fn press(&mut self, key: Key) {
		match key {
			// face
			k if k == self.input.start => self.state.start = true,
			k if k == self.input.a => self.state.a = true,
			k if k == self.input.b => self.state.b = true,
			k if k == self.input.x => self.state.x = true,
			k if k == self.input.y => self.state.y = true,
			k if k == self.input.z => self.state.z = true,

			// triggers
			k if k == self.input.l => {
				self.state.l = true;
				self.state.la = 128;
			},
			k if k == self.input.r => {
				self.state.r = true;
				self.state.ra = 128;
			},
			k if k == self.input.lm => self.state.la = 50,
			k if k == self.input.ls => self.state.la = 22,

			// stick
			k if k == self.input.up => {
				self.state.up = true;
			},
			k if k == self.input.down => {
				self.state.down = true;
			},
			k if k == self.input.left => {
				self.state.left = true;
			},
			k if k == self.input.right => {
				self.state.right = true;
			},

			// c-stick
			k if k == self.input.c_up => {
				self.state.c_up = true;
				self.state.c_vertical = 0;
			},
			k if k == self.input.c_down => {
				self.state.c_down = true;
				self.state.c_vertical = 255;
			},
			k if k == self.input.c_left => {
				self.state.c_left = true;
				self.state.c_horizontal = 0;
			},
			k if k == self.input.c_right => {
				self.state.c_right = true;
				self.state.c_horizontal = 255;
			},

			// modifiers
			k if k == self.input.mod_x => self.state.mod_x = true,
			k if k == self.input.mod_y => self.state.mod_y = true,

			// debug
			k if k == self.input.debug => self.state.debug = true,

			_ => (),
		}
	}

	fn release(&mut self, key: Key) {
		match key {
			// face
			k if k == self.input.start => self.state.start = false,
			k if k == self.input.a => self.state.a = false,
			k if k == self.input.b => self.state.b = false,
			k if k == self.input.x => self.state.x = false,
			k if k == self.input.y => self.state.y = false,
			k if k == self.input.z => self.state.z = false,

			// triggers
			k if k == self.input.l => {
				self.state.l = false;
				self.state.la = 0;
			},
			k if k == self.input.r => {
				self.state.r = false;
				self.state.ra = 0;
			},
			k if k == self.input.lm => self.state.la = 0,
			k if k == self.input.ls => self.state.la = 0,

			// stick
			k if k == self.input.up => {
				self.state.up = false;
			},
			k if k == self.input.down => {
				self.state.down = false;
			},
			k if k == self.input.left => {
				self.state.left = false;
			},
			k if k == self.input.right => {
				self.state.right = false;
			},

			// c-stick
			k if k == self.input.c_up => {
				self.state.c_up = false;
				self.state.c_vertical = 128;
			},
			k if k == self.input.c_down => {
				self.state.c_down = false;
				self.state.c_vertical = 128;
			},
			k if k == self.input.c_left => {
				self.state.c_left = false;
				self.state.c_horizontal = 128;
			},
			k if k == self.input.c_right => {
				self.state.c_right = false;
				self.state.c_horizontal = 128;
			},

			// modifiers
			k if k == self.input.mod_x => self.state.mod_x = false,
			k if k == self.input.mod_y => self.state.mod_y = false,

			_ => (),
		}
	}

	fn process(&mut self) {
		let horizontal = self.state.left || self.state.right;
		let vertical = self.state.up || self.state.down;

		// face
		self.device.send(self.output.start, self.state.start.into()).unwrap();
		self.device.send(self.output.a, self.state.a.into()).unwrap();
		self.device.send(self.output.b, self.state.b.into()).unwrap();
		self.device.send(self.output.x, self.state.x.into()).unwrap();
		self.device.send(self.output.y, self.state.y.into()).unwrap();
		self.device.send(self.output.z, self.state.z.into()).unwrap();

		// triggers
		self.device.send(self.output.l, self.state.l.into()).unwrap();
		self.device.send(self.output.r, self.state.r.into()).unwrap();
		self.device.send(self.output.la, self.state.la.into()).unwrap();
		self.device.send(self.output.ra, self.state.ra.into()).unwrap();

		// stick

		// angles
		if horizontal && vertical {
			if self.state.l || self.state.r {
				// shield
				if self.state.mod_x == self.state.mod_y {
					// shield drops
					if self.state.down {
						self.state.coords.x = 0.725;
						self.state.coords.y = 0.675;
					} else {
						self.state.coords.x = 0.7;
						self.state.coords.y = 0.7;
					}
				} else if self.state.mod_x {
					self.state.coords.x = 0.6375;
					self.state.coords.y = 0.375;
				} else if self.state.mod_y {
					self.state.coords.x = 0.5;
					self.state.coords.y = 0.85;
				}
			} else if self.state.b {
				// b
				if self.state.mod_x == self.state.mod_y {
					self.state.coords.x = 0.59;
					self.state.coords.y = 0.81;
				}
			} else if self.state.mod_x != self.state.mod_y {
				if self.state.mod_x {
					self.state.coords.x = 0.7375;
					self.state.coords.y = 0.3125;
				} else {
					self.state.coords.x = 0.3125;
					self.state.coords.y = 0.7375;
				}
			} else {
				self.state.coords.x = 0.7;
				self.state.coords.y = 0.7;
			}
		} else if horizontal {
			if self.state.mod_x == self.state.mod_y {
				self.state.coords.x = 1.0;
				self.state.coords.y = 0.0;
			} else if self.state.mod_x {
				self.state.coords.x = 0.3;
				self.state.coords.y = 0.0;
			} else {
				self.state.coords.x = 0.45;
				self.state.coords.y = 0.0;
			}
		} else if vertical {
			if self.state.mod_x == self.state.mod_y {
				self.state.coords.x = 0.0;
				self.state.coords.y = 1.0;
			} else if self.state.mod_x {
				self.state.coords.x = 0.0;
				self.state.coords.y = 0.45;
			} else {
				self.state.coords.x = 0.0;
				self.state.coords.y = 0.3;
			}
		} else {
			self.state.coords.set(0.0, 0.0);
		}

		if horizontal {
			// SOCD
			if self.state.right && self.state.left { self.state.coords.x = 0.0 }
			// mirror axis
			if horizontal && !self.state.right { self.state.coords.x = -self.state.coords.x }
		}
		if vertical {
			// SOCD
			if self.state.up && self.state.down { self.state.coords.y = 0.0 }
			// mirror axis
			if !self.state.down { self.state.coords.y = -self.state.coords.y }
		}

		let coord_values = self.state.coords.to_bytes();
		self.device.send(self.output.horizontal, coord_values.0).unwrap();
		self.device.send(self.output.vertical, coord_values.1).unwrap();

		// c-stick
		self.device.send(self.output.c_horizontal, self.state.c_horizontal.into()).unwrap();
		self.device.send(self.output.c_vertical, self.state.c_vertical.into()).unwrap();

		// debug
		if self.state.debug { println!("{:?} => {:?}", self.state.coords, coord_values) }

		self.update();
	}

	fn update(&mut self) {
		self.device.synchronize().unwrap();
	}
}

fn main() {
	let mut rekt = Rekt::new();
	let (send_chan, recv_chan) = channel();

	println!("starting...");
	rekt.process();

	let _listener = thread::spawn(move || {
		listen(move |event| {
			send_chan
				.send(event)
				.unwrap_or_else(|e| println!("Could not send event: {:?}", e));
		})
			.expect("Could not listen")
		;
	});

	loop {
		for event in recv_chan.try_iter() {
			rekt.handle(event);
		}

		rekt.process();
	}
}
