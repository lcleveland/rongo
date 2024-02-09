//! Item rarity
use std::fmt;
#[derive(Debug)]
/// The rarity of an item
pub enum Rarity {
    /// Normal
    Normal,
    /// Magic
    Magic,
    /// Rare
    Rare,
    /// Unique
    Unique,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rarity::Normal => write!(f, "Normal"),
            Rarity::Magic => write!(f, "Magic"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::Unique => write!(f, "Unique"),
        }
    }
}
