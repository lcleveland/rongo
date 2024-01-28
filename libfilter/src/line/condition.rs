use crate::line::gem_quality::GemQuality;
use crate::line::influence::Influence;
use crate::line::operator::Operator;
use crate::line::rarity::Rarity;
#[derive(Debug)]
pub enum Condition {
    AreaLevel(AreaLevel),
    ItemLevel(ItemLevel),
    DropLevel(DropLevel),
    Quality(Quality),
    Rarity(Rarity),
    Class(String),
    BaseType(String),
    Prophecy(String),
    LinkedSockets(u8),
    SocketGroup(SocketGroup),
    Sockets(Sockets),
    Height(Height),
    Width(Width),
    HasExplicitMod(String),
    AnyEnchantment(bool),
    EnchantmentPassiveNode(String),
    EnchantmentPassiveNum(EnchantmentPassiveNum),
    StackSize(StackSize),
    GemLevel(GemLevel),
    GemQualityType(GemQuality),
    AlternativeQuality(bool),
    Replica(bool),
    Identified(bool),
    Corrupted(bool),
    CorruptedMods(CorruptedMods),
    Mirrored(bool),
    ElderItem(bool),
    ShaperItem(bool),
    HasInfluence(Influence),
    FracturedItem(bool),
    SynthesisedItem(bool),
    ElderMap(bool),
    ShapedMap(bool),
    BlightedMap(bool),
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
