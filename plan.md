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

- [ ] Palm rejection classifier (train ML model on touch data to filter accidental palm hits)
- [ ] Virtual pen device (register fake HID device as pen, Usage Page 0x0D Usage 0x02)
- [ ] Convert touch to pen (normalize coordinates, map pressure, handle multi-touch)
- [ ] Stylus tilt/rotation support (if touchpad reports it, map to HID tilt reports)
- [ ] Test with Krita (verify pen recognition and pressure sensitivity work)
- [ ] Performance optimization (reduce latency, add config, handle device disconnect)
