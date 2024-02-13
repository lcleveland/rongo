//! A collection of lines.
use crate::line::Line;
use std::fmt;
/// A collection of lines.
pub struct Block {
    /// The lines in the block.
    pub lines: Vec<Line>,

    /// Whether to hide the items or not.
    pub is_hidden: bool,
}
