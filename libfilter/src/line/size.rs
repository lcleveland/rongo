//! Size of the minimap icon
use std::fmt;
#[derive(Debug)]
/// The size of the minimap icon
pub enum Size {
    /// Disable the minimap icon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Size;
    /// let size = Size::Disable;
    /// # assert_eq!(size.to_string(), "Disable");
    /// ```
    Disable,

    /// Small icon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Size;
    /// let size = Size::Small;
    /// # assert_eq!(size.to_string(), "Small");
    /// ```
    Small,

    /// Medium icon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Size;
    /// let size = Size::Medium;
    /// # assert_eq!(size.to_string(), "Medium");
    /// ```
    Medium,

    /// Largest icon
    ///
    /// # Example
    /// ```
    /// # use libfilter::Size;
    /// let size = Size::Largest;
    /// # assert_eq!(size.to_string(), "Largest");
    /// ```
    Largest,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Size::Disable => write!(f, "Disable"),
            Size::Small => write!(f, "Small"),
            Size::Medium => write!(f, "Medium"),
            Size::Largest => write!(f, "Largest"),
        }
    }
}
