#!/usr/bin/env python

import pynput
import sys
import uinput

def get_key(event):
  try:
    return event.char
  except:
    return event.name

def reset():
  device.emit(uinput.BTN_START, False)
  device.emit(uinput.BTN_A, False)
  device.emit(uinput.BTN_B, False)
  device.emit(uinput.BTN_X, False)
  device.emit(uinput.BTN_Y, False)
  device.emit(uinput.BTN_THUMBR, False)
  device.emit(uinput.BTN_TL, False)
  device.emit(uinput.ABS_RUDDER, 0)
  device.emit(uinput.ABS_RUDDER, 0)
  device.emit(uinput.ABS_RUDDER, 0)
  device.emit(uinput.BTN_TR, False)
  device.emit(uinput.ABS_GAS, 0)
  device.emit(uinput.ABS_Y, 128)
  device.emit(uinput.ABS_Y, 128)
  device.emit(uinput.ABS_X, 128)
  device.emit(uinput.ABS_X, 128)
  device.emit(uinput.ABS_RY, 128)
  device.emit(uinput.ABS_RY, 128)
  device.emit(uinput.ABS_RX, 128)
  device.emit(uinput.ABS_RX, 128)

def on_press(event):
  key = get_key(event)

  if key == "f":
    device.emit(uinput.BTN_START, True)

  elif key == "0":
    device.emit(uinput.BTN_A, True)
  elif key == "4":
    device.emit(uinput.BTN_B, True)
  elif key == "8":
    device.emit(uinput.BTN_X, True)
  elif key == "/":
    device.emit(uinput.BTN_Y, True)
  elif key == "9":
    device.emit(uinput.BTN_THUMBR, True)

  elif key == "a":
    device.emit(uinput.BTN_TL, True)
    device.emit(uinput.ABS_RUDDER, 128)
  elif key == "+":
    device.emit(uinput.ABS_RUDDER, 22)
  elif key == "enter":
    device.emit(uinput.ABS_RUDDER, 64)
  elif key == "7":
    device.emit(uinput.BTN_TR, True)
    device.emit(uinput.ABS_GAS, 128)

  elif key == ".":
    device.emit(uinput.ABS_Y, 0)
  elif key == "e":
    device.emit(uinput.ABS_Y, 255)
  elif key == "o":
    device.emit(uinput.ABS_X, 0)
  elif key == "u":
    device.emit(uinput.ABS_X, 255)

  elif key == "up":
    device.emit(uinput.ABS_RY, 0)
  elif key == "down":
    device.emit(uinput.ABS_RY, 255)
  elif key == "left":
    device.emit(uinput.ABS_RX, 0)
  elif key == "right":
    device.emit(uinput.ABS_RX, 255)

    return True

def on_release(event):
  key = get_key(event)

  if key == "f":
    device.emit(uinput.BTN_START, False)

  elif key == "0":
    device.emit(uinput.BTN_A, False)
  elif key == "4":
    device.emit(uinput.BTN_B, False)
  elif key == "8":
    device.emit(uinput.BTN_X, False)
  elif key == "/":
    device.emit(uinput.BTN_Y, False)
  elif key == "9":
    device.emit(uinput.BTN_THUMBR, False)

  elif key == "a":
    device.emit(uinput.BTN_TL, False)
    device.emit(uinput.ABS_RUDDER, 0)
  elif key == "+":
    device.emit(uinput.ABS_RUDDER, 0)
  elif key == "enter":
    device.emit(uinput.ABS_RUDDER, 0)
  elif key == "7":
    device.emit(uinput.BTN_TR, False)
    device.emit(uinput.ABS_GAS, 0)

  elif key == ".":
    device.emit(uinput.ABS_Y, 128)
  elif key == "e":
    device.emit(uinput.ABS_Y, 128)
  elif key == "o":
    device.emit(uinput.ABS_X, 128)
  elif key == "u":
    device.emit(uinput.ABS_X, 128)

  elif key == "up":
    device.emit(uinput.ABS_RY, 128)
  elif key == "down":
    device.emit(uinput.ABS_RY, 128)
  elif key == "left":
    device.emit(uinput.ABS_RX, 128)
  elif key == "right":
    device.emit(uinput.ABS_RX, 128)

    return True

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

device = uinput.Device(
  events,
  name = "bocks",
  vendor = 0x045e,
  product = 0x028e,
  version = 0x110,
)

# zero
device.emit(uinput.ABS_X, 128, syn = False)

listener = pynput.keyboard.Listener(
  on_press = on_press,
  on_release = on_release,
)

def main():
  reset()
  listener.start()
  listener.join()

if __name__ == "__main__":
  try:
    main()
  except KeyboardInterrupt:
    sys.exit(0)
