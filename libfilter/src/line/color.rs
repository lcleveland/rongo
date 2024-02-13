//! Predefined colors for the minimap icon and light beam
use std::fmt;
#[derive(Debug)]
/// Predefined colors for the minimap icon and light beam
pub enum Color {
    /// Red
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Red;
    /// # assert_eq!(color.to_string(), "Red");
    /// ```
    Red,

    /// Green
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Green;
    /// # assert_eq!(color.to_string(), "Green");
    /// ```
    Green,

    /// Blue
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Blue;
    /// # assert_eq!(color.to_string(), "Blue");
    /// ```
    Blue,

    /// Brown
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Brown;
    /// # assert_eq!(color.to_string(), "Brown");
    /// ```
    Brown,

    /// White
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::White;
    /// # assert_eq!(color.to_string(), "White");
    /// ```
    White,

    /// Yellow
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Yellow;
    /// # assert_eq!(color.to_string(), "Yellow");
    /// ```
    Yellow,

    /// Cyan
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Cyan;
    /// # assert_eq!(color.to_string(), "Cyan");
    /// ```
    Cyan,

    /// Grey
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Grey;
    /// # assert_eq!(color.to_string(), "Grey");
    /// ```
    Grey,

    /// Orange
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Orange;
    /// # assert_eq!(color.to_string(), "Orange");
    /// ```
    Orange,

    /// Pink
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Pink;
    /// # assert_eq!(color.to_string(), "Pink");
    /// ```
    Pink,

    /// Purple
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::Purple;
    /// # assert_eq!(color.to_string(), "Purple");
    /// ```
    Purple,

    /// No color
    ///
    /// Example:
    /// ```
    /// # use libfilter::Color;
    /// let color = Color::None;
    /// # assert_eq!(color.to_string(), "None");
    /// ```
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
