use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EventCode {
    Default = 0,
    NextSong = 163,
    PlayCD = 200,
    PauseCD = 201,
}

impl TryFrom<u16> for EventCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == 0 => Ok(EventCode::Default),
            x if x == 163 => Ok(EventCode::NextSong),
            x if x == 200 => Ok(EventCode::PlayCD),
            x if x == 201 => Ok(EventCode::PauseCD),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}