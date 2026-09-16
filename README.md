# TouchCanvas

TouchCanvas turns a laptop touchpad into a drawing surface for a DIY capacitive
stylus.

The project reads touchpad HID reports, interprets the stylus contact, and
translates it into input for a virtual pen device that drawing applications can
recognise. The goal is to let the user draw with a real stylus on the touchpad
without requiring a graphics tablet.

Built with Rust on Windows using raw HID input from a Windows Precision
Touchpad. The project does not use machine-learning palm rejection: the input
device is intended to be a stylus, and the focus is low-latency touch-to-pen
translation.

The intended drawing workflow is:

```text
DIY stylus → touchpad HID reports → decoded contact data
           → virtual HID pen device → drawing application
```

The system should also be able to suppress or prevent ordinary mouse movement
while drawing mode is active. A future Figma-oriented mode may replace the
current workflow of holding the left mouse button while swiping over the
touchpad. Mode switching and its user experience will be designed later; the
initial implementation will focus on the input pipeline. A dedicated app or
command for enabling and disabling drawing mode is also planned for a later
stage.

## Status

Work in progress — final year CS thesis project.

## Requirements

- Windows 10 or 11
- A laptop with a Windows Precision Touchpad
- A DIY capacitive stylus
- Rust toolchain (rustup.rs)

## Run

cargo run
