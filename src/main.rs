use std::{sync::mpsc::channel, thread};

use rdev::{listen, EventType, Key};
use uinput::{event::{Controller, controller::GamePad, absolute::{self, Wheel, Position}, Absolute}, Device};

struct InputMap {
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
	lm: Key,
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

	// mods
	mod_x: Key,
	mod_y: Key,
}

struct OutputMap {
	// face
	start: GamePad,
	a: GamePad,
	b: GamePad,
	x: GamePad,
	y: GamePad,
	z: GamePad,

	// triggers
	l: GamePad,
	r: GamePad,
	la: Wheel,
	ra: Wheel,

	// stick
	up: Position,
	down: Position,
	left: Position,
	right: Position,

	// c-stick
	c_up: Position,
	c_down: Position,
	c_left: Position,
	c_right: Position,

	// dpad
	d_up: GamePad,
	d_down: GamePad,
	d_left: GamePad,
	d_right: GamePad,
}

struct Rekt {
	device: Device,
	input: InputMap,
	output: OutputMap,
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

			input: InputMap {
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

			output: OutputMap {
				// face
				start: GamePad::Start.into(),
				a: GamePad::A.into(),
				b: GamePad::B.into(),
				x: GamePad::X.into(),
				y: GamePad::Y.into(),
				z: GamePad::ThumbR.into(),

				// triggers
				l: GamePad::TL.into(),
				r: GamePad::TR.into(),
				la: Wheel::Rudder.into(),
				ra: Wheel::Throttle.into(),

				// stick
				up: Position::Y.into(),
				down: Position::Y.into(),
				left: Position::X.into(),
				right: Position::X.into(),

				// c-stick
				c_up: Position::RY.into(),
				c_down: Position::RY.into(),
				c_left: Position::RX.into(),
				c_right: Position::RX.into(),

				// dpad
				d_up: GamePad::North.into(),
				d_down: GamePad::North.into(),
				d_left: GamePad::North.into(),
				d_right: GamePad::North.into(),
			},
		}
	}

	fn process(&mut self, event: rdev::Event) {
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

	fn reset(&mut self) {
		self.device.release(&self.output.start).unwrap();
		self.device.release(&self.output.a).unwrap();
		self.device.release(&self.output.b).unwrap();
		self.device.release(&self.output.x).unwrap();
		self.device.release(&self.output.y).unwrap();
		self.device.release(&self.output.z).unwrap();
		self.device.release(&self.output.l).unwrap();
		self.device.release(&self.output.r).unwrap();
		self.device.position(&self.output.la, 0).unwrap();
		self.device.position(&self.output.ra, 0).unwrap();
		self.device.position(&self.output.up, 128).unwrap();
		self.device.position(&self.output.left, 128).unwrap();
		self.device.position(&self.output.c_up, 128).unwrap();
		self.device.position(&self.output.c_left, 128).unwrap();
	}

	fn press(&mut self, key: Key) {
		match key {
			// face
			k if k == self.input.start => {
				self.device.press(&self.output.start).unwrap();
			},
			k if k == self.input.a => {
				self.device.press(&self.output.a).unwrap();
			},
			k if k == self.input.b => {
				self.device.press(&self.output.b).unwrap();
			},
			k if k == self.input.x => {
				self.device.press(&self.output.x).unwrap();
			},
			k if k == self.input.y => {
				self.device.press(&self.output.y).unwrap();
			},
			k if k == self.input.z => {
				self.device.press(&self.output.z).unwrap();
			},

			// triggers
			k if k == self.input.l => {
				self.device.press(&self.output.l).unwrap();
				self.device.position(&self.output.la, 128).unwrap();
			},
			k if k == self.input.lm => {
				self.device.position(&self.output.la, 50).unwrap();
			},
			k if k == self.input.ls => {
				self.device.position(&self.output.la, 22).unwrap();
			},
			k if k == self.input.r => {
				self.device.press(&self.output.r).unwrap();
				self.device.position(&self.output.ra, 128).unwrap();
			},

			// stick
			k if k == self.input.up => {
				self.device.position(&self.output.up, 0).unwrap();
			},
			k if k == self.input.down => {
				self.device.position(&self.output.down, 255).unwrap();
			},
			k if k == self.input.left => {
				self.device.position(&self.output.left, 0).unwrap();
			},
			k if k == self.input.right => {
				self.device.position(&self.output.right, 255).unwrap();
			},

			// c-stick
			k if k == self.input.c_up => {
				self.device.position(&self.output.c_up, 0).unwrap();
			},
			k if k == self.input.c_down => {
				self.device.position(&self.output.c_down, 255).unwrap();
			},
			k if k == self.input.c_left => {
				self.device.position(&self.output.c_left, 0).unwrap();
			},
			k if k == self.input.c_right => {
				self.device.position(&self.output.c_right, 255).unwrap();
			},

			_ => (),
		}
	}

	fn release(&mut self, key: Key) {
		match key {
			// face
			k if k == self.input.start => {
				self.device.release(&self.output.start).unwrap();
			},
			k if k == self.input.a => {
				self.device.release(&self.output.a).unwrap();
			},
			k if k == self.input.b => {
				self.device.release(&self.output.y).unwrap();
			},
			k if k == self.input.x => {
				self.device.release(&self.output.x).unwrap();
			},
			k if k == self.input.y => {
				self.device.release(&self.output.y).unwrap();
			},
			k if k == self.input.z => {
				self.device.release(&self.output.z).unwrap();
			},

			// triggers
			k if k == self.input.l => {
				self.device.release(&self.output.l).unwrap();
				self.device.position(&self.output.la, 0).unwrap();
			},
			k if k == self.input.lm => {
				self.device.position(&self.output.la, 0).unwrap();
			},
			k if k == self.input.ls => {
				self.device.position(&self.output.la, 0).unwrap();
			},
			k if k == self.input.r => {
				self.device.release(&self.output.r).unwrap();
				self.device.position(&self.output.la, 0).unwrap();
			},

			// stick
			k if k == self.input.up => {
				self.device.position(&self.output.up, 128).unwrap();
			},
			k if k == self.input.down => {
				self.device.position(&self.output.down, 128).unwrap();
			},
			k if k == self.input.left => {
				self.device.position(&self.output.left, 128).unwrap();
			},
			k if k == self.input.right => {
				self.device.position(&self.output.right, 128).unwrap();
			},

			// c-stick
			k if k == self.input.c_up => {
				self.device.position(&self.output.c_up, 128).unwrap();
			},
			k if k == self.input.c_down => {
				self.device.position(&self.output.c_down, 128).unwrap();
			},
			k if k == self.input.c_left => {
				self.device.position(&self.output.c_left, 128).unwrap();
			},
			k if k == self.input.c_right => {
				self.device.position(&self.output.c_right, 128).unwrap();
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
	rekt.reset();

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
