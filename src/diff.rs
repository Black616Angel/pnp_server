use pnp_client::json::*;

use crate::ROOT;
pub struct Diff {}

impl Diff {
    pub fn write_diff(game_name: String, diff: DiffJson) -> Result<String, String> {
        let contents =
            Self::read_file(&(format!("{}games/{}/DefaultScene.json", ROOT.clone(), game_name)));
        let res: Result<DefaultSceneJson, serde_json::Error> = serde_json::from_str(&contents);
        if let Ok(json) = res {
        } else {
            return Err("Error with json in file".to_string());
        }
        Ok("".to_string())
    }

    fn read_file(filename: &str) -> String {
        return std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    }
}
