//! RGBA
use std::fmt;
#[derive(Debug)]
/// Defines colors with an alpha channel.
pub struct RGBA {
    /// The red value.
    pub r: u8,
    /// The green value.
    pub g: u8,
    /// The blue value.
    pub b: u8,
    /// The alpha value.
    pub a: u8,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.r, self.g, self.b, self.a)
    }
}
