# TouchCanvas

TouchCanvas turns your laptop touchpad into a drawing surface.

No graphics tablet needed. No extra hardware. Just your laptop, a DIY
capacitive stylus you can build in five minutes, and this software.

Built with Rust on Windows, using raw HID input from the Windows Precision
Touchpad API, a machine learning palm rejection classifier, and a virtual
pen device that drawing apps like Krita recognise natively.

## Status

Work in progress — final year CS thesis project.

## Requirements

- Windows 10 or 11
- A laptop with a Windows Precision Touchpad
- Rust toolchain (rustup.rs)

## Run

cargo run
