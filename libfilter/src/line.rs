//! A line in a filter

pub mod action;
pub mod color;
pub mod condition;
pub mod gem_quality;
pub mod influence;
pub mod operator;
pub mod rarity;
pub mod rgba;
pub mod shape;
pub mod size;
use std::fmt;

#[derive(Debug)]
/// The lowest component of a filter
///
/// A line is either a condition or an action
///
/// # Example
///
/// `Condition`
/// ```
/// # use libfilter::line::condition::Condition;
/// # use libfilter::line::action;
/// # use libfilter::Operator;
/// # use libfilter::line::Line;
/// let line = Line::Condition(Condition::AreaLevel((Operator::GreaterThan, 1)));
/// # assert_eq!(line.to_string(), "AreaLevel > 1");
/// ```
pub enum Line {
    /// A condition is used as a test to filter items
    Condition(condition::Condition),

    /// An action is what happens when a condition matches an item
    Action(action::Action),
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::Condition(condition) => write!(f, "{}", condition.to_string()),
            Line::Action(action) => write!(f, "{}", action.to_string()),
        }
    }
}
