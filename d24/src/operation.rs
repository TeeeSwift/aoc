use crate::E;
use std::{collections::HashMap, str::FromStr};

impl Operation {
    pub fn operate(
        &self,
        input1: &String,
        input2: &String,
        dest: &String,
        map: &mut HashMap<String, usize>,
    ) -> Result<usize, E> {
        match self {
            Self::AND => {
                let result = map.get(input1).ok_or(E {})? & map.get(input2).ok_or(E {})?;
                map.insert(dest.clone(), result);
                Ok(result)
            }
            Self::OR => {
                let result = map.get(input1).ok_or(E {})? | map.get(input2).ok_or(E {})?;
                map.insert(dest.clone(), result);
                Ok(result)
            }
            Self::XOR => {
                let result = map.get(input1).ok_or(E {})? ^ map.get(input2).ok_or(E {})?;
                map.insert(dest.clone(), result);
                Ok(result)
            }
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(binary_string: &str) -> Result<Self, Self::Err> {
        match binary_string.to_uppercase().as_str() {
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "XOR" => Ok(Self::XOR),
            _ => Err(format!("Invalid operation: {}", binary_string)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    AND,
    OR,
    XOR,
}
