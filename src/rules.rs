/// Structure defining available game rules
use std::collections::HashMap;
use crate::Dict;

pub enum Rules {
    SUTOM(char, u8)  // First letter, word size
}

pub struct DictRegistry (
    HashMap<Rules, Dict>
);

impl DictRegistry {
    pub fn new() -> Self {
        DictRegistry(HashMap::new())
    }
}

