use std::fmt;

// Event types.
// See https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/input-event-codes.h#n34
#[derive(Debug, PartialEq)]
pub enum EventType {
    Syn = 0,
    Key = 1,
}

impl TryFrom<u16> for EventType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == 0 => Ok(EventType::Syn),
            x if x == 1 => Ok(EventType::Key),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}