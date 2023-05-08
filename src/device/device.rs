use regex::Regex;

use crate::errors::IncompleteBuilderError;

// Device input event descriptor.
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub handler: String,
}

// Device input event descriptor builder.
#[derive(Debug)]
struct DeviceBuilder {
    name: Option<String>,
    handler: Option<String>,
}

// Some functions used for chaining, don't wanna delete them.
#[allow(dead_code)]
impl DeviceBuilder {
    pub fn new() -> Self {
        DeviceBuilder { name: None, handler: None }
    }

    pub fn add_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_name_by_ref(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn add_handler(mut self, handler: String) -> Self {
        self.handler = Some(handler);
        self
    }
    
    pub fn add_handler_by_ref(&mut self, handler: String) {
        self.handler = Some(handler);
    }

    pub fn build(self) -> Result<Device, IncompleteBuilderError> {
        if self.name.is_none() {
            return Err(IncompleteBuilderError {});
        }

        if self.handler.is_none() {
            return Err(IncompleteBuilderError {});
        }

        Ok(
            Device {
                name: self.name.unwrap(),
                handler: self.handler.unwrap()
            }
        )
    }
}

impl Device {
    // Create device descriptor from /proc/bus/input/devices-format string.
    pub fn from_proc_bus_input_devices_string(s: &str) -> Result<Self, IncompleteBuilderError> {
        let mut device_builder = DeviceBuilder::new();

        for line in s.lines() {
            match line.chars().nth(0).unwrap() {
                'N' => {
                    match Device::extract_name_from_line(line) {
                        Ok(s) => device_builder.add_name_by_ref(s),
                        Err(_) => continue
                    }
                },
                'H' => {
                    match Device::extract_handler_from_line(line) {
                        Ok(handler) => {
                            if handler.len() != 0 {
                                device_builder.add_handler_by_ref(handler);
                            }
                        }
                        Err(_) => continue
                    }
                },
                _ => ()
            }
        }

        device_builder.build()
    }

    // Extracts name from /proc/bus/input/devices-format string.
    fn extract_name_from_line(line: &str) -> Result<String, ()> {
        // Rust regex don't support lookups soo yeah.
        if !line.starts_with("N: Name=\"") || !line.ends_with("\"") {
            return Err(());
        }

        Ok(line[9..line.len() - 1].to_string())
    }

    // Extracts handler filename from /proc/bus/input/devices-format string.
    fn extract_handler_from_line(line: &str) -> Result<String, ()> {
        // Rust regex don't support lookups soo yeah.
        if !line.starts_with("H: Handlers=") {
            return Err(());
        }

        let event_re = Regex::new("event\\d+").unwrap();

        Ok(event_re.captures(line).unwrap().get(0).unwrap().as_str().to_string())
    }

}