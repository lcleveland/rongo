//! A collection of lines.
use crate::line::Line;
use itertools::Itertools;
use std::fmt;
/// A collection of lines.
///
/// # Example
/// ```
/// # use libfilter::block::Block;
/// # use libfilter::line::Line;
/// # use libfilter::line::condition::Condition;
/// # use libfilter::Operator;
/// let block = Block {
///     lines: vec![
///         Line::Condition(Condition::AreaLevel((libfilter::Operator::GreaterThan, 1))),
///         Line::Condition(Condition::AreaLevel((libfilter::Operator::GreaterThan, 1))),
///         Line::Condition(Condition::AreaLevel((libfilter::Operator::GreaterThan, 1))),
///         ],
///         is_hidden: false
///         };
/// # assert_eq!(block.to_string(), "Show\n\tAreaLevel > 1\n\tAreaLevel > 1\n\tAreaLevel > 1");
/// ```
pub struct Block {
    /// The lines in the block.
    pub lines: Vec<Line>,

    /// Whether to hide the items or not.
    pub is_hidden: bool,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut block_string = String::new();
        if self.is_hidden {
            block_string.push_str("Hide\n\t");
        } else {
            block_string.push_str("Show\n\t");
        }
        block_string.push_str(&self.lines.iter().join("\n\t"));
        write!(f, "{}", block_string)
    }
}
