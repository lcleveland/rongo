//! Predefined shapes for the minimap icon
use std::fmt;
#[derive(Debug)]
/// Predefined shapes for the minimap icon
pub enum Shape {
    /// Circle
    Circle,
    /// Diamond
    Diamond,
    /// Hexagon
    Hexagon,
    /// Square
    Square,
    /// Star
    Star,
    /// Triangle
    Triangle,
    /// Cross
    Cross,
    /// Moon
    Moon,
    /// Raindrop
    Raindrop,
    /// Kite
    Kite,
    /// Pentagon
    Pentagon,
    /// Upside down house
    UpsideDownHouse,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle => write!(f, "Circle"),
            Shape::Diamond => write!(f, "Diamond"),
            Shape::Hexagon => write!(f, "Hexagon"),
            Shape::Square => write!(f, "Square"),
            Shape::Star => write!(f, "Star"),
            Shape::Triangle => write!(f, "Triangle"),
            Shape::Cross => write!(f, "Cross"),
            Shape::Moon => write!(f, "Moon"),
            Shape::Raindrop => write!(f, "Raindrop"),
            Shape::Kite => write!(f, "Kite"),
            Shape::Pentagon => write!(f, "Pentagon"),
            Shape::UpsideDownHouse => write!(f, "UpsideDownHouse"),
        }
    }
}
