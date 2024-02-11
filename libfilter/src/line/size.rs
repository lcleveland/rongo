//! Size of the minimap icon
use std::fmt;
#[derive(Debug)]
/// The size of the minimap icon
pub enum Size {
    /// Disable the minimap icon
    Disable,
    /// Small icon
    Small,
    /// Medium icon
    Medium,
    /// Largest icon
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
