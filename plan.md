# TouchCanvas Plan

## DONE

- [x] Device discovery & detection (HID enumeration, find Synaptics syna30b0, verify via HID capabilities)
  ```rust
  let hid_guid = HidD_GetHidGuid();
  if is_touchpad(&device_path_str) {
      println!("TOUCHPAD FOUND: {}", device_path_str);
  }
  ```
- [x] Read touchpad input data (Raw Input Api and Window message sending extract X/Y/pressure/finger count)

## TODO

- [ ] Decode touchpad HID reports into contact data (contact ID, touch state, X/Y, pressure, and any available stylus-relevant fields)
- [ ] Track the DIY stylus contact across successive reports
- [ ] Normalize touchpad coordinates and map them to virtual pen coordinates
- [ ] Define the pen report format (tip, position, pressure, buttons, and report IDs)
- [ ] Implement a virtual HID pen device (Usage Page `0x0D`, Usage `0x02`)
- [ ] Determine the Windows virtual-HID mechanism required to expose the pen device to applications
- [ ] Convert decoded stylus contact data into pen reports
- [ ] Decide and implement multi-touch behaviour (initially ignore or reject additional contacts)
- [ ] Add stylus tilt and rotation support if the touchpad reports usable values
- [ ] Suppress or block ordinary mouse movement while drawing mode is active
- [ ] Test pen recognition, pressure sensitivity, and drawing behaviour in Krita
- [ ] Test the drawing workflow in Figma without requiring the user to hold the left mouse button
- [ ] Design a reliable mode switch between normal touchpad operation and drawing mode
- [ ] Add a future command or application for enabling and disabling drawing mode
- [ ] Optimize performance (reduce latency and report overhead)
- [ ] Add configuration for coordinate mapping and input behaviour
- [ ] Handle touchpad disconnects and reconnects

## FUTURE UX

- [ ] Define how users enter and leave drawing mode without accidentally disrupting normal touchpad use
- [ ] Ensure normal touchpad gestures and mouse movement resume when drawing mode is disabled
- [ ] Replace the Figma workflow of holding the left mouse button while swiping with explicit drawing-mode behaviour
- [ ] Build a dedicated app or command for controlling drawing mode
