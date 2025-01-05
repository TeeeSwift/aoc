use std::collections::HashMap;

use crate::operation::*;
use crate::E;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    pub input1: String,
    pub input2: String,
    pub op: Operation,
    pub dest: String,
}

impl Ord for Gate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare based on the first character of input1
        self.input1.chars().next().cmp(&other.input1.chars().next())
    }
}

impl PartialOrd for Gate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // Delegate to the full Ord implementation
    }
}

impl Gate {
    pub fn execute(&self, map: &mut HashMap<String, usize>) -> Result<usize, E> {
        self.op.operate(&self.input1, &self.input2, &self.dest, map)
    }
}
