//! A line in a filter

pub mod action;
pub mod condition;
pub mod gem_quality;
pub mod influence;
pub mod operator;
pub mod rarity;
pub mod rgba;
pub mod size;
#[derive(Debug)]
/// The lowest component of a filter
pub enum Line {
    /// A condition is used as a test to filter items
    Condition,

    /// An action is what happens when a condition matches an item
    Action,
}

impl Line {
    /// Function that turns a line into a string
    pub fn to_string(&self) -> String {
        match self {
            Line::Condition => "Condition".to_string(),
            Line::Action => "Action".to_string(),
        }
    }
}
