//! A collection of blocks.
use crate::block::Block;
use itertools::Itertools;
use std::fmt;
/// A collection of blocks.
///
/// # Example
/// ```
/// # use libfilter::section::Section;
/// # use libfilter::block::Block;
/// # use libfilter::line::condition::Condition;
/// # use libfilter::line::action::Action;
/// # use libfilter::RGBA;
/// # use libfilter::Operator;
/// # use libfilter::line::Line;
/// let section = Section {
///     header: "###FFFF".to_string(),
///     description: "###Customize what players see when an item drops in the game.".to_string(),
///     blocks: vec![
///         Block {
///             lines: vec![
///                 Line::Condition(Condition::AreaLevel((Operator::GreaterThan, 1))),
///                 Line::Condition(Condition::Identified(true)),
///                 Line::Action(Action::SetBorderColor(RGBA{r: 255, g: 0, b: 0, a: 255})),
///                 ],
///                 is_hidden: false
///                 },
///                 ],
///                 };
/// # assert_eq!(section.to_string(), "###FFFF\n###Customize what players see when an item drops in the game.\nShow\n\tAreaLevel > 1\n\tIdentified true\n\tSetBorderColor 255 0 0 255");
/// ```
pub struct Section {
    /// Four digit code that can be used to jump around in the filter.
    pub header: String,

    /// The blocks in the section.
    pub blocks: Vec<Block>,

    /// The description of the section.
    pub description: String,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut section_string = String::new();
        section_string.push_str(&self.header);
        section_string.push_str("\n");
        section_string.push_str(&self.description);
        section_string.push_str("\n");
        section_string.push_str(&self.blocks.iter().join("\n"));
        write!(f, "{}", section_string)
    }
}
