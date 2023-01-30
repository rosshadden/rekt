use std::f64::consts::PI;

use rdev::Key;
use uinput::event::{controller::GamePad, absolute::{Wheel, Position}};

pub struct InputMap {
	// face
	pub start: Key,
	pub a: Key,
	pub b: Key,
	pub x: Key,
	pub y: Key,
	pub z: Key,

	// triggers
	pub l: Key,
	pub r: Key,
	pub lm: Key,
	pub ls: Key,

	// stick
	pub up: Key,
	pub down: Key,
	pub left: Key,
	pub right: Key,

	// c-stick
	pub c_up: Key,
	pub c_down: Key,
	pub c_left: Key,
	pub c_right: Key,

	// mods
	pub mod_x: Key,
	pub mod_y: Key,

	// debug
	pub debug: Key,
}

pub struct OutputMap {
	// face
	pub start: GamePad,
	pub a: GamePad,
	pub b: GamePad,
	pub x: GamePad,
	pub y: GamePad,
	pub z: GamePad,

	// triggers
	pub l: GamePad,
	pub r: GamePad,
	pub la: Wheel,
	pub ra: Wheel,

	// stick
	pub horizontal: Position,
	pub vertical: Position,

	// c-stick
	pub c_horizontal: Position,
	pub c_vertical: Position,

	// dpad
	pub d_up: GamePad,
	pub d_down: GamePad,
	pub d_left: GamePad,
	pub d_right: GamePad,
}

#[derive(Clone, Copy, Debug)]
pub struct Coords {
	pub x: f64,
	pub y: f64,
}

impl Coords {
	pub fn set(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}

	pub fn set_x(&mut self, x: f64) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f64) {
		self.y = y;
	}

	pub fn set_vec(&mut self, angle: f64, length: Option<f64>) {
		let len = length.unwrap_or(1.0);
		let sincos = angle.sin_cos();
		self.set(sincos.1 * len, sincos.0 * len);
	}

	pub fn set_deg(&mut self, degrees: f64, length: Option<f64>) {
		self.set_vec(degrees * PI / 180.0, length);
	}

	pub fn to_bytes(self) -> (i32, i32) {
		(
			((self.x * 128.0) + 128.0) as i32,
			((self.y * 128.0) + 128.0) as i32 - 1,
		)
	}
}

pub struct State {
	// face
	pub start: bool,
	pub a: bool,
	pub b: bool,
	pub x: bool,
	pub y: bool,
	pub z: bool,

	// triggers
	pub l: bool,
	pub r: bool,
	pub la: u8,
	pub ra: u8,

	// stick
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub coords: Coords,

	// c-stick
	pub c_up: bool,
	pub c_down: bool,
	pub c_left: bool,
	pub c_right: bool,
	pub c_coords: Coords,

	// dpad
	pub d_up: bool,
	pub d_down: bool,
	pub d_left: bool,
	pub d_right: bool,

	// mods
	pub mod_x: bool,
	pub mod_y: bool,

	// debug
	pub debug: bool,
}

impl State {
	pub fn new() -> Self {
		Self {
			start: false,
			a: false,
			b: false,
			x: false,
			y: false,
			z: false,

			l: false,
			r: false,
			la: 0,
			ra: 0,

			up: false,
			down: false,
			left: false,
			right: false,
			coords: Coords { x: 0.0, y: 0.0 },

			c_up: false,
			c_down: false,
			c_left: false,
			c_right: false,
			c_coords: Coords { x: 0.0, y: 0.0 },

			d_up: false,
			d_down: false,
			d_left: false,
			d_right: false,

			mod_x: false,
			mod_y: false,

			debug: false,
		}
	}
}
