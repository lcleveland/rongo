//! RGBA
use std::fmt;
#[derive(Debug)]
/// Defines colors with an alpha channel.
///
/// # Examples
/// ```
/// use libfilter::RGBA;
/// let rgba = RGBA { r: 0, g: 0, b: 0, a: 255 };
/// # assert_eq!(rgba.to_string(), "0 0 0 255");
/// ```
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
