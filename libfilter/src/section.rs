//! A collection of blocks.
use crate::block::Block;
/// A collection of blocks.
pub struct Section {
    /// Four digit code that can be used to jump around in the filter.
    pub header: String,

    /// The blocks in the section.
    pub blocks: Vec<Block>,

    /// The description of the section.
    pub description: String,
}
