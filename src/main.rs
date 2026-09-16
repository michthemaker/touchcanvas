use std::mem;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Devices::HumanInterfaceDevice::{
	HID_USAGE_PAGE_DIGITIZER,
	HID_USAGE_DIGITIZER_TOUCH_PAD
};
use windows::core::*;

/// This is a message only window, we use it to receive OS messages
/// No visible UI — HWND_MESSAGE (-3) is the magic parent value Windows uses for them
const HWND_MESSAGE_PARENT: HWND = HWND(-3isize as _);

/// The event handler
/// called whenever something happens to the Invisible Window
unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	match msg {
		// windows sends WM_INPUT (255) when raw input is received
		WM_INPUT => {
			unsafe { handle_raw_input(lparam) };
			LRESULT(0)
		}
		WM_DESTROY => {
			// window is being destroyed
			unsafe { PostQuitMessage(0) };
			LRESULT(0)
		}
		// anything else — let Windows handle it the default way
		_ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
	}
}

/// lparam is passed to functions to get raw input data
unsafe fn handle_raw_input(lparam: LPARAM) {
	let header_size = mem::size_of::<RAWINPUTHEADER>() as u32;

	// First call: ask Windows how big the buffer needs to be
	let mut size = 0u32;
	unsafe {
		GetRawInputData(
			HRAWINPUT(lparam.0 as _),
			RID_INPUT,
			None,
			&mut size,
			header_size,
		)
	};
	if size == 0 {
		return;
	}

	// Second call: now that we know the size, allocate and actually read the data
	let mut buffer = vec![0u8; size as usize];
	let written = unsafe {
		GetRawInputData(
			HRAWINPUT(lparam.0 as _),
			RID_INPUT,
			Some(buffer.as_mut_ptr() as *mut _),
			&mut size,
			header_size,
		)
	};
	if written != size {
		return;
	}

	// Reinterpret the raw bytes as a RAWINPUT struct
	let raw = unsafe { &*(buffer.as_ptr() as *const RAWINPUT) };

	// RIM_TYPEHID means this is a generic HID device (not keyboard/mouse) — our touchpad
	if raw.header.dwType == RIM_TYPEHID.0 {
		let hid = unsafe { &raw.data.hid };
		let report_size = hid.dwSizeHid as usize;
		let count = hid.dwCount as usize;

		// bRawData is a variable-length trailing array — walk it via raw pointer
		let data_ptr = hid.bRawData.as_ptr();
		for i in 0..count {
			let report =
				unsafe { std::slice::from_raw_parts(data_ptr.add(i * report_size), report_size) };
			println!("Report ({report_size} bytes): {report:?}");
		}
	}
}

// how to turn [0u8; 30] to `Contact` type

fn main() -> Result<()> {
	unsafe {
		let instance: HMODULE = GetModuleHandleW(None)?;
		let class_name = w!("TouchCanvasRawInputWindow");

		// Register a window class — blueprint for the window we're about to create
		let wc = WNDCLASSW {
			lpfnWndProc: Some(wndproc),
			hInstance: instance.into(),
			lpszClassName: class_name,
			..Default::default()
		};
		RegisterClassW(&wc);

		// Create a message-only window — no UI, just a target for WM_INPUT messages
		let hwnd = CreateWindowExW(
			Default::default(),
			class_name,
			w!("TouchCanvas Raw Input"),
			Default::default(),
			0,
			0,
			0,
			0,
			HWND_MESSAGE_PARENT,
			None,
			instance,
			None,
		)?;

		// Tell Windows: send us raw input from touchpad devices (0x0D = Digitizers, 0x05 = Touch Pad)
		let rid = RAWINPUTDEVICE {
			usUsagePage: HID_USAGE_PAGE_DIGITIZER,
			usUsage: HID_USAGE_DIGITIZER_TOUCH_PAD,
			// RIDEV_INPUTSINK — receive input even when our window isn't in the foreground
			dwFlags: RIDEV_INPUTSINK,
			hwndTarget: hwnd,
		};
		RegisterRawInputDevices(&[rid], mem::size_of::<RAWINPUTDEVICE>() as u32)?;

		let mut msg = MSG::default();
		// Standard Windows message loop — blocks here using Windows kernel sleep, dispatching events to wndproc
		while GetMessageW(&mut msg, None, 0, 0).into() {
			let _ = TranslateMessage(&msg);
			// somehow next line makes windows invoke our wndproc function
			DispatchMessageW(&msg);
		}
	}
	Ok(())
}
