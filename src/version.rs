use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroU8;

use serde::Serializer;

const MINIMUM_MAJOR: u8 = 7;
const MINIMUM_MINOR: NonZeroU8 = NonZeroU8::MIN;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Game {
    Path(Patch),
    #[default]
    Latest,
    Pbe,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Path(version) => version.fmt(f),
            Game::Latest => f.write_str("latest"),
            Game::Pbe => f.write_str("pbe"),
        }
    }
}

impl From<Game> for String {
    fn from(value: Game) -> Self {
        value.to_string()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Version cannot be less than {MINIMUM_MAJOR}.{MINIMUM_MINOR}, but was {0}.{1}")]
pub struct UnsupportedPatchError(u8, u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Patch {
    major: u8,
    minor: NonZeroU8,
}

impl Patch {
    /// Create a new Patch version
    ///
    /// # Errors
    /// Returns `UnsupportedPatchError` if the path version is unsupported by community dragon.
    pub fn new(major: u8, minor: u8) -> Result<Self, UnsupportedPatchError> {
        match (major, NonZeroU8::new(minor)) {
            (MINIMUM_MAJOR.., Some(minor)) => Ok(Patch { major, minor }),
            _ => Err(UnsupportedPatchError(major, minor)),
        }
    }

    #[must_use] pub fn get(&self) -> (u8, u8) {
        (self.major, self.minor.get())
    }

    #[must_use] pub fn major(&self) -> u8 {
        self.major
    }
    
    #[must_use] pub fn minor(&self) -> u8 {
        self.minor.get()
    }
}

impl From<Patch> for String {
    fn from(value: Patch) -> Self {
        value.to_string()
    }
}

impl Display for Patch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_u8(self.major)?;
        f.write_char('.')?;
        f.serialize_u8(self.minor.get())
    }
}
