//! The quality of a gem.
use std::fmt;
#[derive(Debug)]
/// The quality of a gem.
pub enum GemQuality {
    /// The gem is superior
    ///
    /// Example:
    /// ```
    /// # use libfilter::GemQuality;
    /// let quality = GemQuality::Superior;
    /// # assert_eq!(quality.to_string(), "Superior");
    /// ```
    Superior,

    /// The gem is divergent
    ///
    /// Example:
    /// ```
    /// # use libfilter::GemQuality;
    /// let quality = GemQuality::Divergent;
    /// # assert_eq!(quality.to_string(), "Divergent");
    /// ```
    Divergent,

    /// The gem is anomalous
    ///
    /// Example:
    /// ```
    /// # use libfilter::GemQuality;
    /// let quality = GemQuality::Anomalous;
    /// # assert_eq!(quality.to_string(), "Anomalous");
    /// ```
    Anomalous,

    /// The gem is phantasmal
    ///
    /// Example:
    /// ```
    /// # use libfilter::GemQuality;
    /// let quality = GemQuality::Phantasmal;
    /// # assert_eq!(quality.to_string(), "Phantasmal");
    /// ```
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
