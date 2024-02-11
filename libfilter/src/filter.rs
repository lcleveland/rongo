//! Customizes what players see when an item drops in the game.
use crate::section::Section;

/// Customizes what players see when an item drops in the game.
pub struct Filter {
    /// The sections of the filter.
    pub sections: Vec<Section>,
}
