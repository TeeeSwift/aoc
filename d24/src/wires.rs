use std::collections::HashMap;

#[derive(Debug)]
pub struct Wires {
    pub map: HashMap<String, usize>,
}

impl Wires {
    pub fn new(map: HashMap<String, usize>) -> Self {
        Self { map }
    }
}
