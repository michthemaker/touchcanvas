# To-Do List

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

- [ ] Palm rejection classifier
  - [ ] Decode raw HID reports into contact fields (contact ID, touch state, X/Y, width/height, pressure, and confidence where available)
  - [ ] Track contacts across successive reports and maintain per-contact history
  - [ ] Extract classification features (contact area, pressure, movement, duration, edge position, and nearby contacts)
  - [ ] Add logging/visualization so finger and palm samples can be inspected
  - [ ] Implement and tune a rule-based classifier first
  - [ ] Apply classifications to input handling (reject palms from cursor and gesture processing)
  - [ ] Collect labelled finger/palm samples and evaluate false positives and false negatives
  - [ ] Consider training an ML model only after the decoded data and rule-based baseline are reliable
- [ ] Virtual pen device (register fake HID device as pen, Usage Page 0x0D Usage 0x02)
- [ ] Convert touch to pen (normalize coordinates, map pressure, handle multi-touch)
- [ ] Stylus tilt/rotation support (if touchpad reports it, map to HID tilt reports)
- [ ] Test with Krita (verify pen recognition and pressure sensitivity work)
- [ ] Performance optimization (reduce latency, add config, handle device disconnect)
