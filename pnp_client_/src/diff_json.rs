use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use serde::{Serialize, Deserialize};
use crate::scene_json::*;

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct DiffJson {
    pub hash: String,
    pub timestamp: String,
    pub diff: Vec<DiffJsonObject>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[serde(tag = "object_type", content = "change")]
pub enum DiffJsonObject {
    Scene(DiffJsonScene),
    Token(DiffTokenChange),
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[serde(tag = "value_name", content = "new_value")]
#[allow(non_camel_case_types)]
pub enum DiffJsonScene {
    name(String),
    height(i32),
    width(i32),
    square_size(i32),
    texture_background(Option<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[serde(tag = "value_name", content = "new_value")]
#[allow(non_camel_case_types)]
pub enum DiffTokenChange {
    name(String),
    description(String),
    texture_path(String),
    stats(SceneJsonStat),
    StatsNew(SceneJsonStat),
    height(i32),
    width(i32),
    position_x(i32),
    position_y(i32),
}

impl DiffJson {
    pub fn new(old: SceneJson, new: SceneJson) -> Self {
        // simple Macro to push all changed members 
        macro_rules! diffpush_members {
            ($diff:expr, $member:ident) => {
                if new.$member != old.$member {
                    $diff.push(DiffJsonObject::Scene(DiffJsonScene::$member(new.$member)));
                }
            };
        }
        let mut hasher = DefaultHasher::new();
        new.hash(&mut hasher);
        let hash = hasher.finish().to_string();
        let timestamp = "".to_string();

        let mut diff: Vec<DiffJsonObject> = Vec::new();

        diffpush_members!(diff, height);
        diffpush_members!(diff, name);
        diffpush_members!(diff, square_size);
        diffpush_members!(diff, width);
        diffpush_members!(diff, texture_background);


        return Self { hash, timestamp, diff }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
// #[macro_export]