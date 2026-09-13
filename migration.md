# TouchCanvas — Switch HID capture from `CreateFileW`/`ReadFile` to Raw Input API

## Why this change

The current implementation opens the touchpad's HID device path directly
(`CreateFileW` with `FILE_GENERIC_READ`) and reads reports with `ReadFile`.
This fails with `ERROR_SHARING_VIOLATION` (`HRESULT(0x80070020)`):

> "The process cannot access the file because it is being used by another process."

Root cause: Windows' own Precision Touchpad driver stack already holds this
HID device's data channel open exclusively — it's what turns raw contacts
into cursor movement, scrolling, and gestures system-wide. Sharing violations
are decided by what the _first_ opener granted, and the OS driver did not
grant sharing on the data channel. No read-access flag combination on our
end can open it — a metadata-only handle (access `0`) works fine (that's why
device enumeration succeeds), but any handle requesting actual read access
to the report stream will always be refused while the native driver is
attached.

Disabling the native touchpad driver would free the handle but kill system
cursor/gesture handling while the app runs — not acceptable. The correct fix
is to stop trying to open the device ourselves and instead read from the
same report stream Windows is already tapped into, via the **Raw Input API**.

## What changes

**Remove:**

- `CreateFileW` + `ReadFile` + overlapped I/O loop in `read_touchpad_reports`.
- The `SetupDiGetClassDevsW` / `SetupDiEnumDeviceInterfaces` /
  `SetupDiGetDeviceInterfaceDetailW` device-path enumeration, and the
  `is_touchpad` probe that opens each candidate device — Raw Input's own
  device filtering (by usage page/usage) replaces this, so we no longer need
  to identify or open a specific device path at all.

**Add:**

- A message-only window (`HWND_MESSAGE` parent) with a `WndProc`.
- `RegisterRawInputDevices`, filtered to Usage Page `0x0D` (Digitizers),
  Usage `0x05` (Touch Pad). This usage value is what already distinguishes a
  touchpad from other digitizer-class devices like a touchscreen (usage
  `0x04`), so no separate device-path matching is needed.
- A `WM_INPUT` handler that calls `GetRawInputData`, extracts the raw HID
  report bytes from the `RAWHID` payload, and prints/forwards them the same
  way the current code does.
- A standard `GetMessageW`/`DispatchMessageW` loop.

## Cargo.toml

Keep `windows` and `windows-core`. Feature flags needed (exact names may
shift slightly by crate version — check against the version already pinned
in `Cargo.toml`):

- `Win32_Foundation`
- `Win32_UI_WindowsAndMessaging` (window class, message loop)
- `Win32_UI_Input` and/or `Win32_UI_Input_KeyboardAndMouse` (Raw Input lives
  under one of these depending on crate version — `RAWINPUTDEVICE`,
  `RegisterRawInputDevices`, `RAWINPUT`, `RAWINPUTHEADER`, `RAWHID`,
  `RIM_TYPEHID`, `RIDEV_INPUTSINK`, `GetRawInputData` should all resolve
  once the right feature is on)
- `Win32_Devices_HumanInterfaceDevice` (keep — still useful if you later
  want `HidP_GetCaps`/`HidP_GetData` to parse fields instead of printing
  raw bytes)
- `Win32_System_LibraryLoader` (for `GetModuleHandleW`)

`piping`, `windows::core::PCWSTR` usage, etc. from the old enumeration code
can be dropped along with `is_touchpad`.

## Code skeleton

This is a structural skeleton, not guaranteed to compile as-is — module
paths inside the `windows` crate move between versions, so treat import
errors as things to resolve against the pinned crate version's docs, not as
signs the overall approach is wrong.

```rust
use std::mem;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Input::*; // RAWINPUT, RAWINPUTHEADER, RAWHID, RIM_TYPEHID, GetRawInputData
use windows::Win32::UI::Input::KeyboardAndMouse::*; // RAWINPUTDEVICE, RegisterRawInputDevices, RIDEV_*
use windows::core::*;

const HWND_MESSAGE_PARENT: HWND = HWND(-3isize as _);

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_INPUT => {
            handle_raw_input(lparam);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn handle_raw_input(lparam: LPARAM) {
    let mut size = 0u32;
    let header_size = mem::size_of::<RAWINPUTHEADER>() as u32;

    GetRawInputData(HRAWINPUT(lparam.0), RID_INPUT, None, &mut size, header_size);
    if size == 0 {
        return;
    }
    let mut buffer = vec![0u8; size as usize];
    let written = GetRawInputData(
        HRAWINPUT(lparam.0),
        RID_INPUT,
        Some(buffer.as_mut_ptr() as *mut _),
        &mut size,
        header_size,
    );
    if written != size {
        return;
    }

    let raw = &*(buffer.as_ptr() as *const RAWINPUT);
    if raw.header.dwType == RIM_TYPEHID.0 {
        let hid = &raw.data.hid;
        let report_size = hid.dwSizeHid as usize;
        let count = hid.dwCount as usize;
        let data_ptr = hid.bRawData.as_ptr();
        for i in 0..count {
            let report = std::slice::from_raw_parts(data_ptr.add(i * report_size), report_size);
            println!("Report ({report_size} bytes): {report:?}");
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        let instance: HMODULE = GetModuleHandleW(None)?;
        let class_name = w!("TouchCanvasRawInputWindow");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            Default::default(),
            class_name,
            w!("TouchCanvas Raw Input"),
            Default::default(),
            0, 0, 0, 0,
            HWND_MESSAGE_PARENT,
            None,
            instance,
            None,
        )?;

        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x0D, // Digitizers
            usUsage: 0x05,     // Touch Pad
            dwFlags: RIDEV_INPUTSINK, // receive input even without focus
            hwndTarget: hwnd,
        };
        RegisterRawInputDevices(&[rid], mem::size_of::<RAWINPUTDEVICE>() as u32)?;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}
```

## Notes / things to verify while implementing

- `RIDEV_INPUTSINK` requires `hwndTarget` to be set and lets you receive
  input even when the window isn't foreground — needed since this is a
  message-only window with no visible UI.
- `RAWINPUT.data.hid.bRawData` is a variable-length trailing array; the
  fixed-size array in the Rust binding may need the same "index past the
  declared bound via raw pointer" treatment as the old `DevicePath` field —
  this is expected and safe here since `dwCount * dwSizeHid` bytes are
  guaranteed present by the API contract.
- If multiple touch-pad-usage devices are ever attached simultaneously,
  `raw.header.hDevice` identifies which physical device a given `WM_INPUT`
  message came from — not needed for a single built-in Precision Touchpad,
  but worth knowing if that assumption changes later.
- Confirm `windows` crate version in `Cargo.toml` and check docs.rs for that
  exact version if any of the above symbols don't resolve where listed —
  Raw Input types have moved between `Win32::UI::Input` and
  `Win32::UI::Input::KeyboardAndMouse` across crate releases.

## Acceptance check

Run `cargo run`. It should no longer panic with `ERROR_SHARING_VIOLATION`,
and moving fingers on the Precision Touchpad should produce continuous
`Report (N bytes): [...]` output — while the touchpad continues to control
the system cursor normally at the same time.
