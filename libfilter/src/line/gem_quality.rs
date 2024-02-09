use std::fmt;
#[derive(Debug)]
pub enum GemQuality {
    Superior,
    Divergent,
    Anomalous,
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
