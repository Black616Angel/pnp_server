use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StringVecJson {
    pub count: usize,
    pub values: Vec<String>,
}

impl StringVecJson {
    pub fn from_vec(vec: Vec<String>) -> Self {
        Self {
            count: vec.len(),
            values: vec,
        }
    }
}
