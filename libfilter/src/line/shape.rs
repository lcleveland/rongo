//! Predefined shapes for the minimap icon
use std::fmt;
#[derive(Debug)]
/// Predefined shapes for the minimap icon
pub enum Shape {
    /// Circle
    ///
    /// # Examples
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Circle;
    /// # assert_eq!(shape.to_string(), "Circle");
    /// ```
    Circle,

    /// Diamond
    ///
    /// # Examples
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Diamond;
    /// # assert_eq!(shape.to_string(), "Diamond");
    /// ```
    Diamond,

    /// Hexagon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Hexagon;
    /// # assert_eq!(shape.to_string(), "Hexagon");
    /// ```
    Hexagon,

    /// Square
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Square;
    /// # assert_eq!(shape.to_string(), "Square");
    /// ```
    Square,

    /// Star
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Star;
    /// # assert_eq!(shape.to_string(), "Star");
    /// ```
    Star,

    /// Triangle
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Triangle;
    /// # assert_eq!(shape.to_string(), "Triangle");
    /// ```
    Triangle,

    /// Cross
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Cross;
    /// # assert_eq!(shape.to_string(), "Cross");
    /// ```
    Cross,

    /// Moon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Moon;
    /// # assert_eq!(shape.to_string(), "Moon");
    /// ```
    Moon,

    /// Raindrop
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Raindrop;
    /// # assert_eq!(shape.to_string(), "Raindrop");
    /// ```
    Raindrop,

    /// Kite
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Kite;
    /// # assert_eq!(shape.to_string(), "Kite");
    /// ```
    Kite,

    /// Pentagon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::Pentagon;
    /// # assert_eq!(shape.to_string(), "Pentagon");
    /// ```
    Pentagon,

    /// Upside down house
    ///
    /// # Example
    /// ```
    /// # use libfilter::Shape;
    /// let shape = Shape::UpsideDownHouse;
    /// # assert_eq!(shape.to_string(), "UpsideDownHouse");
    /// ```
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
