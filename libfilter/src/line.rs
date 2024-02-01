//! A line in a filter

pub mod action;
pub mod condition;
pub mod gem_quality;
pub mod influence;
pub mod operator;
pub mod rarity;
pub mod rgba;
#[derive(Debug)]
/// The lowest component of a filter
pub enum Line {
    Condition,
    Action,
}
