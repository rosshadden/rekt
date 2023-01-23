#!/usr/bin/env python

import pynput
import sys
import uinput

def get_key(event):
  try:
    return event.char
  except:
    return event.name

class Blocks:

  keymap = {
    # face
    "START": "f",
    "A": "0",
    "B": "4",
    "X": "8",
    "Y": "/",
    "Z": "9",

    # triggers
    "L": "a",
    "R": "7",
    "MS": "enter",
    "LS": "+",

    # stick
    "UP": ".",
    "DOWN": "e",
    "LEFT": "o",
    "RIGHT": "u",

    # c-stick
    "C-UP": "up",
    "C-DOWN": "down",
    "C-LEFT": "left",
    "C-RIGHT": "right",

    # dpad
    "D-UP": "",
    "D-DOWN": "",
    "D-LEFT": "",
    "D-RIGHT": "",

    # mods
    "MOD-X": "k",
    "MOD-Y": "space",
  }

  def init(self):
    events = (
      uinput.BTN_START,
      uinput.BTN_A,
      uinput.BTN_B,
      uinput.BTN_X,
      uinput.BTN_Y,
      uinput.BTN_THUMBR,
      uinput.BTN_TL,
      uinput.BTN_TR,
      uinput.ABS_RUDDER + (-128, 128, 0, 0),
      uinput.ABS_GAS + (-128, 128, 0, 0),
      uinput.ABS_X + (0, 255, 0, 0),
      uinput.ABS_Y + (0, 255, 0, 0),
      uinput.ABS_RX + (0, 255, 0, 0),
      uinput.ABS_RY + (0, 255, 0, 0),
    )

    self.device = uinput.Device(
      events,
      name = "bocks",
      vendor = 0x045e,
      product = 0x028e,
      version = 0x110,
    )

    self.reset()

  def reset(self):
    self.device.emit(uinput.BTN_START, False, syn = False)
    self.device.emit(uinput.BTN_A, False, syn = False)
    self.device.emit(uinput.BTN_B, False, syn = False)
    self.device.emit(uinput.BTN_X, False, syn = False)
    self.device.emit(uinput.BTN_Y, False, syn = False)
    self.device.emit(uinput.BTN_THUMBR, False, syn = False)
    self.device.emit(uinput.BTN_TL, False, syn = False)
    self.device.emit(uinput.ABS_RUDDER, 0, syn = False)
    self.device.emit(uinput.ABS_RUDDER, 0, syn = False)
    self.device.emit(uinput.ABS_RUDDER, 0, syn = False)
    self.device.emit(uinput.BTN_TR, False, syn = False)
    self.device.emit(uinput.ABS_GAS, 0, syn = False)
    self.device.emit(uinput.ABS_Y, 128, syn = False)
    self.device.emit(uinput.ABS_Y, 128, syn = False)
    self.device.emit(uinput.ABS_X, 128, syn = False)
    self.device.emit(uinput.ABS_X, 128, syn = False)
    self.device.emit(uinput.ABS_RY, 128, syn = False)
    self.device.emit(uinput.ABS_RY, 128, syn = False)
    self.device.emit(uinput.ABS_RX, 128, syn = False)
    self.device.emit(uinput.ABS_RX, 128, syn = False)

  def on_press(self):
    def on_press(event):
      key = get_key(event)

      # face
      if key == self.keymap["START"]:
        self.device.emit(uinput.BTN_START, True)
      elif key == self.keymap["A"]:
        self.device.emit(uinput.BTN_A, True)
      elif key == self.keymap["B"]:
        self.device.emit(uinput.BTN_B, True)
      elif key == self.keymap["X"]:
        self.device.emit(uinput.BTN_X, True)
      elif key == self.keymap["Y"]:
        self.device.emit(uinput.BTN_Y, True)
      elif key == self.keymap["Z"]:
        self.device.emit(uinput.BTN_THUMBR, True)

      # triggers
      if key == self.keymap["L"]:
        self.device.emit(uinput.BTN_TL, True)
        self.device.emit(uinput.ABS_RUDDER, 128)
      elif key == self.keymap["MS"]:
        self.device.emit(uinput.ABS_RUDDER, 50)
      elif key == self.keymap["LS"]:
        self.device.emit(uinput.ABS_RUDDER, 22)
      elif key == self.keymap["R"]:
        self.device.emit(uinput.BTN_TR, True)
        self.device.emit(uinput.ABS_GAS, 128)

      # stick
      if key == self.keymap["UP"]:
        self.device.emit(uinput.ABS_Y, 0)
      elif key == self.keymap["DOWN"]:
        self.device.emit(uinput.ABS_Y, 255)
      elif key == self.keymap["LEFT"]:
        self.device.emit(uinput.ABS_X, 0)
      elif key == self.keymap["RIGHT"]:
        self.device.emit(uinput.ABS_X, 255)

      # c-stick
      if key == self.keymap["C-UP"]:
        self.device.emit(uinput.ABS_RY, 0)
      elif key == self.keymap["C-DOWN"]:
        self.device.emit(uinput.ABS_RY, 255)
      elif key == self.keymap["C-LEFT"]:
        self.device.emit(uinput.ABS_RX, 0)
      elif key == self.keymap["C-RIGHT"]:
        self.device.emit(uinput.ABS_RX, 255)

    return on_press

  def on_release(self):
    def on_release(event):
      key = get_key(event)

      # face
      if key == self.keymap["START"]:
        self.device.emit(uinput.BTN_START, False)
      elif key == self.keymap["A"]:
        self.device.emit(uinput.BTN_A, False)
      elif key == self.keymap["B"]:
        self.device.emit(uinput.BTN_B, False)
      elif key == self.keymap["X"]:
        self.device.emit(uinput.BTN_X, False)
      elif key == self.keymap["Y"]:
        self.device.emit(uinput.BTN_Y, False)
      elif key == self.keymap["Z"]:
        self.device.emit(uinput.BTN_THUMBR, False)

      # triggers
      if key == self.keymap["L"]:
        self.device.emit(uinput.BTN_TL, False)
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == self.keymap["MS"]:
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == self.keymap["LS"]:
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == self.keymap["R"]:
        self.device.emit(uinput.BTN_TR, False)
        self.device.emit(uinput.ABS_GAS, 0)

      # stick
      if key == self.keymap["UP"]:
        self.device.emit(uinput.ABS_Y, 128)
      elif key == self.keymap["DOWN"]:
        self.device.emit(uinput.ABS_Y, 128)
      elif key == self.keymap["LEFT"]:
        self.device.emit(uinput.ABS_X, 128)
      elif key == self.keymap["RIGHT"]:
        self.device.emit(uinput.ABS_X, 128)

      # c-stick
      if key == self.keymap["C-UP"]:
        self.device.emit(uinput.ABS_RY, 128)
      elif key == self.keymap["C-DOWN"]:
        self.device.emit(uinput.ABS_RY, 128)
      elif key == self.keymap["C-LEFT"]:
        self.device.emit(uinput.ABS_RX, 128)
      elif key == self.keymap["C-RIGHT"]:
        self.device.emit(uinput.ABS_RX, 128)

    return on_release

  def start(self):
    listener = pynput.keyboard.Listener(
      on_press = self.on_press(),
      on_release = self.on_release(),
    )

    listener.start()
    listener.join()

def main():
  blocks = Blocks()
  blocks.init()
  blocks.start()

if __name__ == "__main__":
  try:
    main()
  except KeyboardInterrupt:
    sys.exit(0)
