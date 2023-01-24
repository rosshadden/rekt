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
				x: Key::Kp8,
				y: Key::KpDivide,
				z: Key::Kp9,

				// triggers
				l: Key::KeyA,
				r: Key::Kp7,
				lm: Key::KpReturn,
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

				// mods
				mod_x: Key::KeyK,
				mod_y: Key::Space,
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
			EventType::KeyPress(key) => {
				self.press(key);
			},
			EventType::KeyRelease(key) => {
				self.release(key);
			},
			_ => (),
		}

		self.process();
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
				self.state.vertical = 0;
			},
			k if k == self.input.down => {
				self.state.down = true;
				self.state.vertical = 255;
			},
			k if k == self.input.left => {
				self.state.left = true;
				self.state.horizontal = 0;
			},
			k if k == self.input.right => {
				self.state.right = true;
				self.state.horizontal = 255;
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
				self.state.vertical = 128;
			},
			k if k == self.input.down => {
				self.state.down = false;
				self.state.vertical = 128;
			},
			k if k == self.input.left => {
				self.state.left = false;
				self.state.horizontal = 128;
			},
			k if k == self.input.right => {
				self.state.right = false;
				self.state.horizontal = 128;
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

			_ => (),
		}
	}

	fn process(&mut self) {
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
		self.device.send(self.output.horizontal, self.state.horizontal.into()).unwrap();
		self.device.send(self.output.vertical, self.state.vertical.into()).unwrap();

		// c-stick
		self.device.send(self.output.c_horizontal, self.state.c_horizontal.into()).unwrap();
		self.device.send(self.output.c_vertical, self.state.c_vertical.into()).unwrap();

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

	for event in recv_chan.iter() {
		rekt.handle(event);
	}
}
