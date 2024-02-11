//! Predefined colors for the minimap icon and light beam
use std::fmt;
#[derive(Debug)]
/// Predefined colors for the minimap icon and light beam
pub enum Color {
    /// Red
    Red,
    /// Green
    Green,
    /// Blue
    Blue,
    /// Brown
    Brown,
    /// White
    White,
    /// Yellow
    Yellow,
    /// Cyan
    Cyan,
    /// Grey
    Grey,
    /// Orange
    Orange,
    /// Pink
    Pink,
    /// Purple
    Purple,
    /// No color
    None,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Brown => write!(f, "Brown"),
            Color::White => write!(f, "White"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Cyan => write!(f, "Cyan"),
            Color::Grey => write!(f, "Grey"),
            Color::Orange => write!(f, "Orange"),
            Color::Pink => write!(f, "Pink"),
            Color::Purple => write!(f, "Purple"),
            Color::None => write!(f, "None"),
        }
    }
}
