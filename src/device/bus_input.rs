use std::fs::read_to_string;
use std::io;

use super::device::Device;

// Get list of available input devices.
pub fn get_input_device_list() -> io::Result<Vec<Device>> {
    let raw_content = read_to_string("/proc/bus/input/devices")?;

    let mut input_device_list = vec![];

    for raw_device_info in raw_content.split("\n\n") {
        let device = match Device::from_proc_bus_input_devices_string(raw_device_info) {
            Ok(d) => d,
            Err(_) => { continue; }
        };

        input_device_list.push(device);
    }

    Ok(input_device_list)
}

// Find JBL GO device input file.
pub fn find_jbl_device_input_file(devices: Vec<Device>) -> Option<String> {
    for device in devices {
        if device.name.contains("JBL GO") {
            return Some(device.handler);
        }
    }

    None
}