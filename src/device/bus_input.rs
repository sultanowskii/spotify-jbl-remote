use std::fs::read_to_string;

use crate::errors::exit_with_error;
use super::device::Device;

// Get list of available input devices.
fn get_input_device_list() -> Vec<Device> {
    let raw_content = match read_to_string("/proc/bus/input/devices") {
        Ok(f) => f,
        Err(e) => exit_with_error(format!("Error occurred trying to open a /proc/bus/input/devices: {}", e).as_str())
    };

    let mut input_device_list = vec![];

    for raw_device_info in raw_content.split("\n\n") {
        let device = match Device::from_proc_bus_input_devices_string(raw_device_info) {
            Ok(d) => d,
            Err(_) => { continue; }
        };

        input_device_list.push(device);
    }

    input_device_list
}

// Find JBL GO device input file.
pub fn find_jbl_device_input_file() -> Option<String> {
    for device in get_input_device_list() {
        if device.name.contains("JBL GO") {
            return Some(device.handler);
        }
    }

    None
}