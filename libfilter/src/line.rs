pub mod action;
pub mod condition;
pub mod gem_quality;
pub mod influence;
pub mod operator;
pub mod rarity;
#[derive(Debug)]
pub enum Line {
    Condition,
    Action,
}
