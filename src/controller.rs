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
