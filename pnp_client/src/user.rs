use crate::fs::File;
use crate::params::PROGRAM_PARAMETERS;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub uid: String,
    pub session_id: String,
    games: Vec<String>,
}

impl User {
    pub async fn get() -> Self {
        for param in PROGRAM_PARAMETERS.clone() {
            if param.key == "sid" {
                return Self::from_session_id(param.value).await;
            }
        }
        //
        panic!();
    }
    pub async fn from_session_id(session_id: String) -> Self {
        File::read_sid(session_id).await
    }
}
