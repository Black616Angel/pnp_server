use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct UserJson {
    pub uuid: String,
    pub name: String,
    pub games: Vec<GamesJson>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct SessionJson {
    pub session_id: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GamesJson {
    pub uid: String,
    pub name: String,
}
