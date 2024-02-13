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
    ///
    /// `String`: The name of the base type to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let base_type = Condition::BaseType(String::from("Iron Ring"));
    /// # assert_eq!(base_type.to_string(), "BaseType \"Iron Ring\"");
    /// ```
    BaseType(String),

    /// The prophecy name. Specifying a part of a prophecy name is allowed and will match any of the prophecies with that text in the name. Prophecies have the Class type "Stackable Currency".
    ///
    /// `String`: The name of the prophecy to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let prophecy = Condition::Prophecy(String::from("Twice Enchanted"));
    /// # assert_eq!(prophecy.to_string(), "Prophecy \"Twice Enchanted\"");
    /// ```
    Prophecy(String),

    /// The size of the largest group of linked sockets that the item has.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let linked_sockets = Condition::LinkedSockets((Operator::GreaterThan, 1));
    /// # assert_eq!(linked_sockets.to_string(), "LinkedSockets > 1");
    /// ```
    LinkedSockets((Operator, u8)),

    /// Supports a list of groups that each one represents linked sockets containing a specific set of colors, at least one group must be matched for the condition to pass.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `String`: The color sequence to test.
    /// `u8`: The longest link on the item.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let socket_group = Condition::SocketGroup((Operator::GreaterThanOrEqual, String::from("GGG"), 5));
    /// # assert_eq!(socket_group.to_string(), "SocketGroup >= 5GGG");
    /// ```
    SocketGroup((Operator, String, u8)),

    /// Does the exact same thing as [SocketGroup] but does not require the sockets to be linked.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `String`: The color sequence to test.
    /// `u8`: The number of sockets on the item.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let sockets = Condition::Sockets((Operator::GreaterThanOrEqual, String::from("GGG"), 5));
    /// # assert_eq!(sockets.to_string(), "Sockets >= 5GGG");
    /// ```
    Sockets((Operator, String, u8)),

    /// The number of slots the item takes on the Y-axis (verical axis), i.e. the height of the item.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let height = Condition::Height((Operator::GreaterThan, 1));
    /// # assert_eq!(height.to_string(), "Height > 1");
    /// ```
    Height((Operator, u8)),

    /// The number of slots the item takes on the X-axis (horizontal axis), i.e. the width of the item.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let width = Condition::Width((Operator::GreaterThan, 1));
    /// # assert_eq!(width.to_string(), "Width > 1");
    /// ```
    Width((Operator, u8)),

    /// Filter by mods on an item by name.
    ///
    /// `String`: The name of the mod to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let mod_name = Condition::HasExplicitMod(String::from("Tyrannical"));
    /// # assert_eq!(mod_name.to_string(), "HasExplicitMod \"Tyrannical\"");
    /// ```
    HasExplicitMod(String),

    /// If an item has any enchantment from the Labyrinth.
    ///
    /// `bool`: If the item has any enchantment from the Labyrinth.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let any_enchantment = Condition::AnyEnchantment(true);
    /// # assert_eq!(any_enchantment.to_string(), "AnyEnchantment true");
    /// ```
    AnyEnchantment(bool),

    /// Filter by enchantments.
    ///
    /// `String`: The name of the enchantment to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let enchantment = Condition::HasEnchantment(String::from("Edict of War"));
    /// # assert_eq!(enchantment.to_string(), "HasEnchantment \"Edict of War\"");
    /// ```
    HasEnchantment(String),

    /// Filter Cluster Jewels by enchantment type.
    ///
    /// `String`: The name of the enchantment to match.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let enchantment = Condition::EnchantmentPassiveNode(String::from("increased Damage with Two Handed Weapons"));
    /// # assert_eq!(enchantment.to_string(), "EnchantmentPassiveNode \"increased Damage with Two Handed Weapons\"");
    /// ```
    EnchantmentPassiveNode(String),

    /// Filter Cluster Jewels by the number of enchantments. Only checks the "Adds X passive skills" modifier.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The number of enchantments to match.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let enchantment = Condition::EnchantmentPassiveNum((Operator::GreaterThan, 1));
    /// # assert_eq!(enchantment.to_string(), "EnchantmentPassiveNum > 1");
    /// ```
    EnchantmentPassiveNum((Operator, u8)),

    /// Currency stack size.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let stack_size = Condition::StackSize((Operator::GreaterThan, 1));
    /// # assert_eq!(stack_size.to_string(), "StackSize > 1");
    /// ```
    StackSize((Operator, u8)),

    /// The level of the gem.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The value to test.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let gem_level = Condition::GemLevel((Operator::GreaterThan, 1));
    /// # assert_eq!(gem_level.to_string(), "GemLevel > 1");
    /// ```
    GemLevel((Operator, u8)),

    /// The [GemQuality] of the gem.
    ///
    /// `GemQuality`: A value from the [GemQuality] enum.
    /// # Example
    /// ```
    /// # use libfilter::GemQuality;
    /// # use libfilter::line::condition::Condition;
    /// let gem_quality = Condition::GemQualityType(GemQuality::Superior);
    /// # assert_eq!(gem_quality.to_string(), "GemQualityType Superior");
    /// ```
    GemQualityType(GemQuality),

    /// If an item has alternate quality or not.
    ///
    /// `bool`: If the item has alternate quality.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let alt_quality = Condition::AlternativeQuality(true);
    /// # assert_eq!(alt_quality.to_string(), "AlternativeQuality true");
    /// ```
    AlternativeQuality(bool),

    /// If an item is an Replica or not.
    ///
    /// `bool`: If the item is an Replica.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let replica = Condition::Replica(true);
    /// # assert_eq!(replica.to_string(), "Replica true");
    /// ```
    Replica(bool),

    /// If an item is identified or not.
    ///
    /// `bool`: If the item is identified.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let identified = Condition::Identified(true);
    /// # assert_eq!(identified.to_string(), "Identified true");
    /// ```
    Identified(bool),

    /// If an item is corrupted or not.
    ///
    /// `bool`: If the item is corrupted.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let corrupted = Condition::Corrupted(true);
    /// # assert_eq!(corrupted.to_string(), "Corrupted true");
    /// ```
    Corrupted(bool),

    /// How many corrupted mods are present.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The number of corrupted mods.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let corrupted_mods = Condition::CorruptedMods((Operator::GreaterThan, 1));
    /// # assert_eq!(corrupted_mods.to_string(), "CorruptedMods > 1");
    /// ```
    CorruptedMods((Operator, u8)),

    /// If an item is mirrored or not.
    ///
    /// `bool`: If the item is mirrored.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let mirrored = Condition::Mirrored(true);
    /// # assert_eq!(mirrored.to_string(), "Mirrored true");
    /// ```
    Mirrored(bool),

    /// If an item is an Elder Item or not.
    ///
    /// `bool`: If the item is an Elder Item.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let elder_item = Condition::ElderItem(true);
    /// # assert_eq!(elder_item.to_string(), "ElderItem true");
    /// ```
    ElderItem(bool),

    /// If an item is a Shaper Item or not.
    ///
    /// `bool`: If the item is a Shaper Item.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let shaper_item = Condition::ShaperItem(true);
    /// # assert_eq!(shaper_item.to_string(), "ShaperItem true");
    /// ```
    ShaperItem(bool),

    /// If an item has an [Influence]
    ///
    /// `Influence`: A value from the [Influence] enum
    /// # Example
    /// ```
    /// # use libfilter::Influence;
    /// # use libfilter::line::condition::Condition;
    /// let influence = Condition::HasInfluence(Influence::Shaper);
    /// # assert_eq!(influence.to_string(), "HasInfluence Shaper");
    /// ```
    HasInfluence(Influence),

    /// If an item is fractured or not.
    ///
    /// `bool`: If the item is fractured.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let fractured_item = Condition::FracturedItem(true);
    /// # assert_eq!(fractured_item.to_string(), "FracturedItem true");
    /// ```
    FracturedItem(bool),

    /// If an item is a Synthesised item or not
    ///
    /// `bool`: If the item is a Synthesised item.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let synthesised_item = Condition::SynthesisedItem(true);
    /// # assert_eq!(synthesised_item.to_string(), "SynthesisedItem true");
    /// ```
    SynthesisedItem(bool),

    /// If the map is elder or not.
    ElderMap(bool),

    /// If the map is Shaped or not.
    ///
    /// `bool`: If the map is Shaped.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let shaped_map = Condition::ShapedMap(true);
    /// # assert_eq!(shaped_map.to_string(), "ShapedMap true");
    /// ```
    ShapedMap(bool),

    /// If the map is Blighted or not.
    ///
    /// `bool`: If the map is Blighted.
    /// # Example
    /// ```
    /// # use libfilter::line::condition::Condition;
    /// let blighted_map = Condition::BlightedMap(true);
    /// # assert_eq!(blighted_map.to_string(), "BlightedMap true");
    /// ```
    BlightedMap(bool),

    /// The map tier of the map.
    ///
    /// `Operator`: A value from the [Operator] enum.
    /// `u8`: The map tier of the map.
    /// # Example
    /// ```
    /// # use libfilter::Operator;
    /// # use libfilter::line::condition::Condition;
    /// let map_tier = Condition::MapTier((Operator::GreaterThan, 1));
    /// # assert_eq!(map_tier.to_string(), "MapTier > 1");
    /// ```
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
                write!(f, "SocketGroup {} {}{}", op, count, val)
            }
            Condition::Sockets((op, val, count)) => write!(f, "Sockets {} {}{}", op, count, val),
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
