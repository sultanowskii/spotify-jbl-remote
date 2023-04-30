use std::fmt;

// Event codes.
// Read https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/input-event-codes.h#n64
#[derive(Debug, PartialEq)]
pub enum EventCode {
    Default,
    NextSong,
    PlayCD,
    PauseCD,
    Unused,
}

impl TryFrom<u16> for EventCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventCode::Default),
            163 => Ok(EventCode::NextSong),
            200 => Ok(EventCode::PlayCD),
            201 => Ok(EventCode::PauseCD),
            _ => Ok(EventCode::Unused),
        }
    }
}

impl fmt::Display for EventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}