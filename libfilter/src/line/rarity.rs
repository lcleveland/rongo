use std::fmt;
#[derive(Debug)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
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
