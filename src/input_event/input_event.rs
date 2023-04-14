use std::fmt;

use crate::errors::InvalidArgumentError;

use super::{EventCode, EventType};

// Input event size.
pub const INPUT_EVENT_CHUNK_SIZE: usize = 24;

// Linux input event structure.
pub struct InputEvent {
    pub time: u64,
    pub dummy: u64, // idk + idc
    pub type_: EventType,
    pub code: EventCode,
    pub value: i32,
}

impl TryFrom<&Vec<u8>> for InputEvent {
    type Error = InvalidArgumentError;

    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() != 24 {
            return Err(InvalidArgumentError {});
        }

        Ok(
            InputEvent {
                time: u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
                dummy: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
                type_: u16::from_le_bytes(bytes[16..18].try_into().unwrap()).try_into().unwrap(),
                code: u16::from_le_bytes(bytes[18..20].try_into().unwrap()).try_into().unwrap(),
                value: i32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            }
        )
    }
}

impl fmt::Display for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event time={}, type={}, code={}, value={}",
            self.time, self.type_, self.code, self.value
        )
    }
}
