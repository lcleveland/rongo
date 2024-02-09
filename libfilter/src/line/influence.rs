//! Influence
use std::fmt;
#[derive(Debug)]
/// Influence
pub enum Influence {
    /// Shaper
    Shaper,
    /// Elder
    Elder,
    /// Crusader
    Crusader,
    /// Hunter
    Hunter,
    /// Redeemer
    Redeemer,
    /// Warlord
    Warlord,
    /// None
    None,
}

impl fmt::Display for Influence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Influence::Shaper => write!(f, "Shaper"),
            Influence::Elder => write!(f, "Elder"),
            Influence::Crusader => write!(f, "Crusader"),
            Influence::Hunter => write!(f, "Hunter"),
            Influence::Redeemer => write!(f, "Redeemer"),
            Influence::Warlord => write!(f, "Warlord"),
            Influence::None => write!(f, "None"),
        }
    }
}
