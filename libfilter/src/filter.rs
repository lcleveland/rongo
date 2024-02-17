//! Customizes what players see when an item drops in the game.
use crate::section::Section;
use std::fs::File;
use std::io::{Error, Write};

/// Customizes what players see when an item drops in the game.
pub struct Filter {
    /// The sections of the filter.
    pub sections: Vec<Section>,
}

impl Filter {
    /// Creates a new filter.
    pub fn new() -> Filter {
        Filter { sections: vec![] }
    }

    /// Adds a section to the filter.
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section)
    }

    /// Writes the filter to a file.
    ///
    /// # Example
    /// ```
    /// # use libfilter::filter::Filter;
    /// # use libfilter::section::Section;
    /// # use libfilter::block::Block;
    /// # use libfilter::line::condition::Condition;
    /// # use libfilter::line::action::Action;
    /// # use libfilter::RGBA;
    /// # use libfilter::Operator;
    /// # use libfilter::line::Line;
    /// let mut filter = Filter::new();
    /// filter.add_section(Section {
    ///     header: "FFFF".to_string(),
    ///     description: "Customize what players see when an item drops in the game.".to_string(),
    ///     blocks: vec![
    ///         Block {
    ///             lines: vec![
    ///                 Line::Condition(Condition::AreaLevel((Operator::GreaterThan, 1))),
    ///                 Line::Condition(Condition::Identified(true)),
    ///                 Line::Action(Action::SetBorderColor(RGBA{r: 255, g: 0, b: 0, a: 255})),
    ///                 ],
    ///             is_hidden: false
    ///         },
    ///     ],
    /// });
    /// filter.write_to_file("filter");
    /// ```
    pub fn write_to_file(&self, path: &str) -> Result<(), Error> {
        let mut file = File::create(path)?;
        for section in &self.sections {
            writeln!(file, "{}", section)?;
        }
        Ok(())
    }
}
