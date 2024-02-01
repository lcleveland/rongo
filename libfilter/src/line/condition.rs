//! Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it

use crate::line::gem_quality::GemQuality;
use crate::line::influence::Influence;
use crate::line::operator::Operator;
use crate::line::rarity::Rarity;
#[derive(Debug)]
/// Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it
pub enum Condition {
    /// Filters for items dropped in a particular Monster level of the current area.
    AreaLevel(AreaLevel),
    /// The item level the item was generated at
    ItemLevel(ItemLevel),
    /// The level that the item starts dropping at
    DropLevel(DropLevel),
    /// The amount of quality on the item
    Quality(Quality),
    /// The [Rarity] of the item
    Rarity(Rarity),
    /// The item class. Specifying part of a class name is allowed and will match any classes with that text in the name
    Class(String),
    /// The base type of the item. Specifying a part of a base type name is allowed and will match any of the base types with that text in the name.
    BaseType(String),
    /// The prophecy name. Specifying a part of a prophecy name is allowed and will match any of the prophecies with that text in the name. Prophecies have the Class type "Stackable Currency".
    Prophecy(String),
    /// The size of the largest group of linked sockets that the item has.
    LinkedSockets(u8),
    /// Supports a list of groups that each one represents linked sockets containing a specific set of colors, at least one group must be matched for the condition to pass.
    SocketGroup(SocketGroup),
    /// Does the exact same thing as [SocketGroup] but does not require the sockets to be linked.
    Sockets(Sockets),
    /// The number of slots the item takes on the Y-axis (verical axis), i.e. the height of the item.
    Height(Height),
    /// The number of slots the item takes on the X-axis (horizontal axis), i.e. the width of the item.
    Width(Width),
    /// Filter by mods on an item by name.
    HasExplicitMod(String),
    /// If an item has any enchantment from the Labyrinth.
    AnyEnchantment(bool),
    /// Filter Cluster Jewels by enchantment type.
    EnchantmentPassiveNode(String),
    /// Filter Cluster Jewels by the number of enchantments. Only checks the "Adds X passive skills" modifier.
    EnchantmentPassiveNum(EnchantmentPassiveNum),
    /// Currency stack size.
    StackSize(StackSize),
    /// The level of the gem.
    GemLevel(GemLevel),
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
    CorruptedMods(CorruptedMods),
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
    MapTier(MapTier),
}

#[derive(Debug)]
pub struct AreaLevel {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct ItemLevel {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct DropLevel {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct Quality {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct SocketGroup {
    pub operator: Operator,
    pub color: String,
    pub longest_link: u8,
}

#[derive(Debug)]
pub struct Sockets {
    pub operator: Operator,
    pub color: String,
    pub number_sockets: u8,
}

#[derive(Debug)]
pub struct Height {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct Width {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct EnchantmentPassiveNum {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct StackSize {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct GemLevel {
    pub operator: Operator,
    pub value: u8,
}

#[derive(Debug)]
pub struct CorruptedMods {
    pub operator: Operator,
    pub value: GemQuality,
}

#[derive(Debug)]
pub struct MapTier {
    pub operator: Operator,
    pub value: String,
}
