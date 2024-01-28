pub mod action;
pub mod condition;
#[derive(Debug)]
pub enum Line {
    Condition,
    Action,
}
