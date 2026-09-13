use std::mem;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Devices::DeviceAndDriverInstallation::*;
use windows::Win32::Devices::HumanInterfaceDevice::*;

// the gist is that Windows logs Hid devices' activities in system specific files

fn main() {
	unsafe {
		let hid_guid = HidD_GetHidGuid();
		let hdevinfo = SetupDiGetClassDevsW(
			Some(&hid_guid),
			None,
			None,
			DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
		)
		.expect("Failed to get device info set");

		let mut device_interface_data = SP_DEVICE_INTERFACE_DATA {
			// Windows needs to know the size of the struct so we do this
			cbSize: mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32,
			..Default::default()
		};

		let mut index = 0;
		while SetupDiEnumDeviceInterfaces(hdevinfo, None, &hid_guid, index, &mut device_interface_data)
			.is_ok()
		{
			// Get detail data size
			let mut required_size = 0u32;
			let _ = SetupDiGetDeviceInterfaceDetailW(
				hdevinfo,
				&device_interface_data,
				None,
				0,
				Some(&mut required_size as *mut u32),
				None,
			);

			// Get the actual detail data
			let mut detail_data = vec![0u8; required_size as usize];
			let detail_ptr = detail_data.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;

			// Set cbSize field
			(*detail_ptr).cbSize = mem::size_of::<u64>() as u32;
			let _ = SetupDiGetDeviceInterfaceDetailW(
				hdevinfo,
				&device_interface_data,
				Some(detail_ptr),
				required_size,
				Some(&mut required_size as *mut u32),
				None,
			);

			// DevicePath is right after cbSize (u32), starts at offset 4 on 32-bit and 8 on 64-bit
			let path_ptr = (detail_data.as_ptr() as usize + 8) as *const u16;
			// Find null terminator
			let mut len = 0;
			while *path_ptr.add(len) != 0 {
				len += 1;
			}

			let path_slice = std::slice::from_raw_parts(path_ptr, len);
			let path = std::ffi::OsString::from_wide(path_slice);
			let device_path_str = path.to_string_lossy();

			if is_touchpad(&device_path_str) {
				println!("TOUCHPAD FOUND: {}", device_path_str);
			}
			index += 1;
		}

		let _ = SetupDiDestroyDeviceInfoList(hdevinfo);
	}
}

unsafe fn is_touchpad(path: &str) -> bool {
	use windows::Win32::Storage::FileSystem::{
		CreateFileW,
		FILE_FLAG_OVERLAPPED,
		// flag for generic read, we just need to access the capabilities
		// flags for share mode of the file
		FILE_SHARE_READ,
		FILE_SHARE_WRITE,
		// what to do about the file already created
		OPEN_EXISTING,
	};
	use windows::core::PCWSTR;

	// path looks like "?\\hid#..." but should be like "\\\\?\\hid#..."
	let fixed_path = if path.starts_with("?\\") {
		format!("\\\\?\\{}", &path[2..])
	} else {
		path.to_string()
	};

	let wide_file_path: Vec<u16> = fixed_path.encode_utf16().chain(std::iter::once(0)).collect();
	// we are opening an existing device system specific file here, hence the OPEN_EXISTING flag to get the file handle.
	let file_handle = unsafe {
		CreateFileW(
			PCWSTR(wide_file_path.as_ptr()),
			// we use 0 and not FILE_GENERIC_READ.0 because the device is system-protected so we don't input spoof.
			// using 0 means we get a metadata-only handle that bypasses the security restriction
			0,
			FILE_SHARE_READ | FILE_SHARE_WRITE,
			None,
			OPEN_EXISTING,
			FILE_FLAG_OVERLAPPED,
			None,
		)
	};

	let file_handle = match file_handle {
		Ok(h) => {
			h
		}
		Err(_) => return false,
	};

	let mut preparsed_data: PHIDP_PREPARSED_DATA = PHIDP_PREPARSED_DATA::default();
	// pull Hid descriptor from device file handle into preparsed_data
	if (unsafe { HidD_GetPreparsedData(file_handle, &mut preparsed_data) }).as_bool() {
		let mut capabilities = HIDP_CAPS::default();
		if (unsafe { HidP_GetCaps(preparsed_data, &mut capabilities) }).is_err() {
			return false;
		}

		// free preparsed data
		unsafe {
			HidD_FreePreparsedData(preparsed_data);
		}
		// Usage Page = 0x0D which is Digitisers and Usage is 0x05 Touch Pad
		capabilities.UsagePage == 0x0D && capabilities.Usage == 0x05
	} else {
		return false;
	}
}
