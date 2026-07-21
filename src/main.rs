use windows::Win32::Devices::HumanInterfaceDevice::*;

fn main() {
	unsafe {
		let hid_guid = HidD_GetHidGuid();
		print!("HiD GUID: {:?}\n", hid_guid);
	}
}
