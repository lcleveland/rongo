//! Item rarity
use std::fmt;
#[derive(Debug)]
/// The rarity of an item
pub enum Rarity {
    /// Normal
    ///
    /// Example:
    /// ```
    /// # use libfilter::Rarity;
    /// let rarity = Rarity::Normal;
    /// # assert_eq!(rarity.to_string(), "Normal");
    /// ```
    Normal,

    /// Magic
    ///
    /// Example:
    /// ```
    /// # use libfilter::Rarity;
    /// let rarity = Rarity::Magic;
    /// # assert_eq!(rarity.to_string(), "Magic");
    /// ```
    Magic,

    /// Rare
    ///
    /// Example:
    /// ```
    /// # use libfilter::Rarity;
    /// let rarity = Rarity::Rare;
    /// # assert_eq!(rarity.to_string(), "Rare");
    /// ```
    Rare,

    /// Unique
    ///
    /// Example:
    /// ```
    /// # use libfilter::Rarity;
    /// let rarity = Rarity::Unique;
    /// # assert_eq!(rarity.to_string(), "Unique");
    /// ```
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
