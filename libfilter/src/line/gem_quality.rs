//! The quality of a gem.
use std::fmt;
#[derive(Debug)]
/// The quality of a gem.
pub enum GemQuality {
    /// The gem is superior
    Superior,
    /// The gem is divergent
    Divergent,
    /// The gem is anomalous
    Anomalous,
    /// The gem is phantasmal
    Phantasmal,
}

impl fmt::Display for GemQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GemQuality::Superior => write!(f, "Superior"),
            GemQuality::Divergent => write!(f, "Divergent"),
            GemQuality::Anomalous => write!(f, "Anomalous"),
            GemQuality::Phantasmal => write!(f, "Phantasmal"),
        }
    }
}
