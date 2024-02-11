//! Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it

use crate::line::gem_quality::GemQuality;
use crate::line::influence::Influence;
use crate::line::operator::Operator;
use crate::line::rarity::Rarity;
use std::fmt;
#[derive(Debug)]
/// Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it
pub enum Condition {
    /// Filters for items dropped in a particular Monster level of the current area.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let area_level = Condition::AreaLevel((Operator::GreaterThan, 1));
    /// # assert_eq!(area_level.to_string(), "AreaLevel > 1");
    /// ```
    AreaLevel((Operator, u8)),

    /// The item level the item was generated at.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let item_level = Condition::ItemLevel((Operator::GreaterThan, 1));
    /// # assert_eq!(item_level.to_string(), "ItemLevel > 1");
    /// ```
    ItemLevel((Operator, u8)),

    /// The level that the item starts dropping at.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let drop_level = Condition::DropLevel((Operator::GreaterThan, 1));
    /// # assert_eq!(drop_level.to_string(), "DropLevel > 1");
    DropLevel((Operator, u8)),

    /// The amount of quality on the item.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let quality = Condition::Quality((Operator::GreaterThan, 1));
    /// # assert_eq!(quality.to_string(), "Quality > 1");
    Quality((Operator, u8)),

    /// The [Rarity] of the item.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `Rarity`: The [Rarity] to test.
    /// # Example
    /// ```
    /// # use libfilter::{Operator, Rarity};
    /// # use libfilter::line::condition::Condition;
    /// let rarity = Condition::Rarity((Operator::GreaterThan, Rarity::Normal));
    /// # assert_eq!(rarity.to_string(), "Rarity > Normal");
    /// ```
    Rarity((Operator, Rarity)),

    /// The item class. Specifying part of a class name is allowed and will match any classes with that text in the name.
    ///
    /// `String`: The name of the class to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let class = Condition::Class(String::from("One Handed"));
    /// # assert_eq!(class.to_string(), "Class \"One Handed\"");
    Class(String),

    /// The base type of the item. Specifying a part of a base type name is allowed and will match any of the base types with that text in the name.
    BaseType(String),

    /// The prophecy name. Specifying a part of a prophecy name is allowed and will match any of the prophecies with that text in the name. Prophecies have the Class type "Stackable Currency".
    Prophecy(String),

    /// The size of the largest group of linked sockets that the item has.
    LinkedSockets((Operator, u8)),

    /// Supports a list of groups that each one represents linked sockets containing a specific set of colors, at least one group must be matched for the condition to pass.
    SocketGroup((Operator, String, u8)),

    /// Does the exact same thing as [SocketGroup] but does not require the sockets to be linked.
    Sockets((Operator, String, u8)),

    /// The number of slots the item takes on the Y-axis (verical axis), i.e. the height of the item.
    Height((Operator, u8)),

    /// The number of slots the item takes on the X-axis (horizontal axis), i.e. the width of the item.
    Width((Operator, u8)),

    /// Filter by mods on an item by name.
    HasExplicitMod(String),

    /// If an item has any enchantment from the Labyrinth.
    AnyEnchantment(bool),

    /// Filter by enchantments.
    HasEnchantment(String),

    /// Filter Cluster Jewels by enchantment type.
    EnchantmentPassiveNode(String),

    /// Filter Cluster Jewels by the number of enchantments. Only checks the "Adds X passive skills" modifier.
    EnchantmentPassiveNum((Operator, u8)),

    /// Currency stack size.
    StackSize((Operator, u8)),

    /// The level of the gem.
    GemLevel((Operator, u8)),

    /// The [GemQuality] of the gem.
    GemQualityType(GemQuality),

    /// If an item has alternate quality or not.
    AlternativeQuality(bool),

    /// If an item is an Replica or not.
    Replica(bool),

    /// If an item is identified or not.
    Identified(bool),

    /// If an item is corrupted or not.
    Corrupted(bool),

    /// How many corrupted mods are present.
    CorruptedMods((Operator, u8)),

    /// If an item is mirrored or not.
    Mirrored(bool),

    /// If an item is an Elder Item or not.
    ElderItem(bool),

    /// If an item is a Shaper Item or not.
    ShaperItem(bool),

    /// If an item has an [Influence]
    HasInfluence(Influence),

    /// If an item is fractured or not.
    FracturedItem(bool),

    /// If an item is a Synthesised item or not
    SynthesisedItem(bool),

    /// If the map is elder or not.
    ElderMap(bool),

    /// If the map is Shaped or not.
    ShapedMap(bool),

    /// If the map is Blighted or not.
    BlightedMap(bool),

    /// The map tier of the map.
    MapTier((Operator, u8)),
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::AreaLevel((op, val)) => write!(f, "AreaLevel {} {}", op, val),
            Condition::ItemLevel((op, val)) => write!(f, "ItemLevel {} {}", op, val),
            Condition::DropLevel((op, val)) => write!(f, "DropLevel {} {}", op, val),
            Condition::Quality((op, quality)) => write!(f, "Quality {} {}", op, quality),
            Condition::Rarity((op, rarity)) => write!(f, "Rarity {} {}", op, rarity),
            Condition::Class(val) => write!(f, "Class \"{}\"", val),
            Condition::BaseType(val) => write!(f, "BaseType \"{}\"", val),
            Condition::Prophecy(val) => write!(f, "Prophecy \"{}\"", val),
            Condition::LinkedSockets((op, val)) => write!(f, "LinkedSockets {} {}", op, val),
            Condition::SocketGroup((op, val, count)) => {
                write!(f, "SocketGroup {} {} {}", op, val, count)
            }
            Condition::Sockets((op, val, count)) => write!(f, "Sockets {} {} {}", op, val, count),
            Condition::Height((op, val)) => write!(f, "Height {} {}", op, val),
            Condition::Width((op, val)) => write!(f, "Width {} {}", op, val),
            Condition::HasExplicitMod(val) => write!(f, "HasExplicitMod \"{}\"", val),
            Condition::AnyEnchantment(val) => write!(f, "AnyEnchantment {}", val),
            Condition::HasEnchantment(val) => write!(f, "HasEnchantment \"{}\"", val),
            Condition::EnchantmentPassiveNode(val) => {
                write!(f, "EnchantmentPassiveNode \"{}\"", val)
            }
            Condition::EnchantmentPassiveNum((op, val)) => {
                write!(f, "EnchantmentPassiveNum {} {}", op, val)
            }
            Condition::StackSize((op, val)) => write!(f, "StackSize {} {}", op, val),
            Condition::GemLevel((op, val)) => write!(f, "GemLevel {} {}", op, val),
            Condition::GemQualityType(val) => write!(f, "GemQualityType {}", val),
            Condition::AlternativeQuality(val) => write!(f, "AlternativeQuality {}", val),
            Condition::Replica(val) => write!(f, "Replica {}", val),
            Condition::Identified(val) => write!(f, "Identified {}", val),
            Condition::Corrupted(val) => write!(f, "Corrupted {}", val),
            Condition::CorruptedMods((op, val)) => write!(f, "CorruptedMods {} {}", op, val),
            Condition::Mirrored(val) => write!(f, "Mirrored {}", val),
            Condition::ElderItem(val) => write!(f, "ElderItem {}", val),
            Condition::ShaperItem(val) => write!(f, "ShaperItem {}", val),
            Condition::HasInfluence(val) => write!(f, "HasInfluence {}", val),
            Condition::FracturedItem(val) => write!(f, "FracturedItem {}", val),
            Condition::SynthesisedItem(val) => write!(f, "SynthesisedItem {}", val),
            Condition::ElderMap(val) => write!(f, "ElderMap {}", val),
            Condition::ShapedMap(val) => write!(f, "ShapedMap {}", val),
            Condition::BlightedMap(val) => write!(f, "BlightedMap {}", val),
            Condition::MapTier((op, val)) => write!(f, "MapTier {} {}", op, val),
        }
    }
}
