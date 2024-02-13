//! Influence
use std::fmt;
#[derive(Debug)]
/// Influence
pub enum Influence {
    /// Shaper
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Shaper;
    /// # assert_eq!(influence.to_string(), "Shaper");
    /// ```
    Shaper,

    /// Elder
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Elder;
    /// # assert_eq!(influence.to_string(), "Elder");
    /// ```
    Elder,

    /// Crusader
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Crusader;
    /// # assert_eq!(influence.to_string(), "Crusader");
    /// ```
    Crusader,

    /// Hunter
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Hunter;
    /// # assert_eq!(influence.to_string(), "Hunter");
    /// ```
    Hunter,

    /// Redeemer
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Redeemer;
    /// # assert_eq!(influence.to_string(), "Redeemer");
    /// ```
    Redeemer,

    /// Warlord
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::Warlord;
    /// # assert_eq!(influence.to_string(), "Warlord");
    /// ```
    Warlord,

    /// None
    ///
    /// Example:
    /// ```
    /// # use libfilter::Influence;
    /// let influence = Influence::None;
    /// # assert_eq!(influence.to_string(), "None");
    /// ```
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
