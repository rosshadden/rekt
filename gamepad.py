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

      if key == "f":
        self.device.emit(uinput.BTN_START, True)

      elif key == "0":
        self.device.emit(uinput.BTN_A, True)
      elif key == "4":
        self.device.emit(uinput.BTN_B, True)
      elif key == "8":
        self.device.emit(uinput.BTN_X, True)
      elif key == "/":
        self.device.emit(uinput.BTN_Y, True)
      elif key == "9":
        self.device.emit(uinput.BTN_THUMBR, True)

      elif key == "a":
        self.device.emit(uinput.BTN_TL, True)
        self.device.emit(uinput.ABS_RUDDER, 128)
      elif key == "+":
        self.device.emit(uinput.ABS_RUDDER, 22)
      elif key == "enter":
        self.device.emit(uinput.ABS_RUDDER, 50)
      elif key == "7":
        self.device.emit(uinput.BTN_TR, True)
        self.device.emit(uinput.ABS_GAS, 128)

      elif key == ".":
        self.device.emit(uinput.ABS_Y, 0)
      elif key == "e":
        self.device.emit(uinput.ABS_Y, 255)
      elif key == "o":
        self.device.emit(uinput.ABS_X, 0)
      elif key == "u":
        self.device.emit(uinput.ABS_X, 255)

      elif key == "up":
        self.device.emit(uinput.ABS_RY, 0)
      elif key == "down":
        self.device.emit(uinput.ABS_RY, 255)
      elif key == "left":
        self.device.emit(uinput.ABS_RX, 0)
      elif key == "right":
        self.device.emit(uinput.ABS_RX, 255)

    return on_press

  def on_release(self):
    def on_release(event):
      key = get_key(event)

      if key == "f":
        self.device.emit(uinput.BTN_START, False)

      elif key == "0":
        self.device.emit(uinput.BTN_A, False)
      elif key == "4":
        self.device.emit(uinput.BTN_B, False)
      elif key == "8":
        self.device.emit(uinput.BTN_X, False)
      elif key == "/":
        self.device.emit(uinput.BTN_Y, False)
      elif key == "9":
        self.device.emit(uinput.BTN_THUMBR, False)

      elif key == "a":
        self.device.emit(uinput.BTN_TL, False)
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == "+":
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == "enter":
        self.device.emit(uinput.ABS_RUDDER, 0)
      elif key == "7":
        self.device.emit(uinput.BTN_TR, False)
        self.device.emit(uinput.ABS_GAS, 0)

      elif key == ".":
        self.device.emit(uinput.ABS_Y, 128)
      elif key == "e":
        self.device.emit(uinput.ABS_Y, 128)
      elif key == "o":
        self.device.emit(uinput.ABS_X, 128)
      elif key == "u":
        self.device.emit(uinput.ABS_X, 128)

      elif key == "up":
        self.device.emit(uinput.ABS_RY, 128)
      elif key == "down":
        self.device.emit(uinput.ABS_RY, 128)
      elif key == "left":
        self.device.emit(uinput.ABS_RX, 128)
      elif key == "right":
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
