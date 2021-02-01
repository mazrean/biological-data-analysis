use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub matchVal: i128,
    pub unmatchVal: i128,
    pub gapVal: i128
}

impl Score {
    pub fn get(&self, x: char, y: char) -> i128 {
        if x == y {
            return self.matchVal
        } else if x == '_' || y == '_' {
            return self.gapVal
        }
        self.unmatchVal
    }
}