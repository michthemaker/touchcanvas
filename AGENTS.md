# Agent Instructions for TouchCanvas

## Project Goal

TouchCanvas turns a laptop Windows Precision Touchpad into a drawing surface
for a DIY capacitive stylus.

The intended input pipeline is:

```text
DIY stylus
    -> touchpad HID reports
    -> decoded contact data
    -> virtual HID pen device
    -> drawing application
```

The project is written in Rust for Windows and is intended to work with
drawing applications such as Krita. The system should eventually allow a user
to draw with the stylus without needing a graphics tablet.

## Current Scope

The core work is:

1. Read raw touchpad HID reports.
2. Decode the device-specific report format.
3. Track the stylus contact across successive reports.
4. Normalize touchpad coordinates.
5. Map contact data to virtual pen reports.
6. Expose a virtual HID pen recognized by Windows and drawing applications.
7. Map pressure where the touchpad provides a usable pressure value.
8. Support stylus tilt or rotation only if the touchpad reports usable values.
9. Suppress ordinary mouse movement while drawing mode is active.
10. Validate pen recognition and drawing behaviour in Krita.

## Explicitly Out of Scope

Do not add a machine-learning palm-rejection classifier. The project assumes
the user is intentionally using a DIY stylus. Do not add training datasets,
palm/finger classification, or ML dependencies unless the project direction is
explicitly changed.

Do not treat palm rejection as a prerequisite for the input pipeline. Focus on
reliable stylus contact decoding and virtual-pen output instead.

## Virtual Pen Direction

The virtual device should represent a pen under the HID Digitizers usage page:

```text
Usage Page: 0x0D (Digitizers)
Usage:      0x02 (Pen)
```

Before implementing the device, determine the correct Windows virtual-HID
mechanism and verify that the resulting device is visible to applications.
Do not assume that registering a raw-input window alone creates a virtual HID
device. Raw Input is the input-reading side; virtual-device creation is a
separate concern.

The pen report format should be designed deliberately and documented before
implementation. At minimum, consider:

- tip/contact state
- X and Y coordinates
- pressure
- supported pen buttons
- report IDs
- optional tilt and rotation fields

## Drawing Modes and UX

Normal touchpad use must remain possible. Drawing mode should eventually be
explicitly enabled and disabled so the user can switch between:

- normal touchpad mouse and gesture behaviour
- stylus drawing behaviour

While drawing mode is active, ordinary mouse movement should be suppressed or
otherwise prevented from interfering with the pen input.

For the Figma workflow, the goal is to remove the need to hold the left mouse
button while swiping over the touchpad. The precise mode-switching interaction
is a later UX feature and should not be guessed prematurely.

A dedicated app or command for enabling and disabling drawing mode is planned
for a later stage. Keep the input pipeline separable from the future UI.

## Implementation Order

Prefer this order unless a technical dependency requires otherwise:

1. Identify and document the actual touchpad HID report descriptor.
2. Decode raw reports into typed contact data.
3. Verify coordinates, contact state, pressure, and report timing with logs.
4. Track one stylus contact and ignore additional contacts initially.
5. Normalize coordinates and define the pen coordinate mapping.
6. Implement the virtual HID pen device.
7. Translate contact data into pen reports.
8. Test recognition and pressure in Krita.
9. Add mouse suppression and drawing-mode control.
10. Investigate tilt and rotation support.
11. Optimize latency and report handling.
12. Add configuration and disconnect/reconnect handling.

## Engineering Guidelines

- Preserve existing Rust and Windows API patterns in the repository.
- Keep raw input acquisition, report decoding, contact tracking, pen output,
  and mode control as separate concerns.
- Use precise types for decoded HID fields rather than passing unstructured
  byte arrays through the whole program.
- Treat the HID report descriptor as authoritative; do not guess byte offsets
  from Microsoft's sample descriptor alone.
- Be explicit about unsafe code and validate buffer sizes and report lengths.
- Surface Windows API errors instead of silently ignoring them.
- Avoid unrelated changes and unnecessary dependencies.
- Keep latency low: avoid blocking work in the Windows input callback.
- Add logging or diagnostics where they help verify report decoding and timing.

## Validation Targets

The implementation should eventually demonstrate:

- the touchpad is detected reliably
- raw reports are decoded correctly
- the DIY stylus produces a stable contact stream
- the virtual device appears as a pen in Windows
- Krita recognizes the device as a pen
- pen movement maps correctly across the drawing surface
- pressure changes affect brush behaviour where supported
- normal mouse movement does not interfere during drawing mode
- normal touchpad behaviour returns when drawing mode is disabled
- disconnects and reconnects are handled without crashing
