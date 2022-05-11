use crate::server_jsons::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub session_id: String,
    uuid: String,
    name: String,
    games: Vec<String>,
}

impl User {
    pub fn read_sids(session_id: String, root_path: String) -> Option<Self> {
        let json = Self::read_file(root_path.clone() + "users/sessions.json");
        let sid_json: Result<Vec<SessionJson>, serde_json::Error> = serde_json::from_str(&json);
        match sid_json {
            Ok(sid_json) => {
                for session in sid_json {
                    if session_id == session.session_id {
                        return Self::read_user(session, root_path.clone());
                    }
                }
                None
            }
            Err(_err) => None,
        }
    }

    fn read_user(session: SessionJson, root_path: String) -> Option<Self> {
        let user_file: Result<UserJson, serde_json::Error> = serde_json::from_str(
            &Self::read_file(format!("{}users/{}.json", root_path, &session.uuid)),
        );
        match user_file {
            Ok(user_file) => {
                let mut games: Vec<String> = Vec::new();
                for game in user_file.games {
                    games.push(game.uid);
                }
                Some(Self {
                    session_id: session.session_id,
                    uuid: session.uuid,
                    name: user_file.name,
                    games,
                })
            }
            Err(_err) => None,
        }
    }

    fn read_file(filename: String) -> String {
        return std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    }
}
