//! Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it

use crate::line::gem_quality::GemQuality;
use crate::line::influence::Influence;
use crate::line::operator::Operator;
use crate::line::rarity::Rarity;
#[derive(Debug)]
/// Conditions are used to determine if an item should be shown or hidden or have an [crate::line::action::Action] applied to it
pub enum Condition {
    /// Filters for items dropped in a particular Monster level of the current area.
    AreaLevel((Operator, u8)),
    /// The item level the item was generated at
    ItemLevel((Operator, u8)),
    /// The level that the item starts dropping at
    DropLevel((Operator, u8)),
    /// The amount of quality on the item
    Quality((Operator, u8)),
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
