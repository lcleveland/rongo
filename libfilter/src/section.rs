use crate::block::Block;
pub struct Section {
    pub header: String,
    pub blocks: Vec<Block>,
    pub description: String,
}
