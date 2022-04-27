use pnp_client::diff_json::*;
use pnp_client::scene_json::*;

pub struct Diff {}

impl Diff {
    pub fn get_diff(game_name: String, hash: String) -> String {
        let ret = Diff::read_diffs(game_name, hash);
        if let Ok(ret) = ret {
            return ret.to_string();
        } else {
            let ret = ret.unwrap_err();
            return serde_json::ser::to_string(&ret).unwrap();
        }
        
    }

    fn read_diffs(game_name: String, hash: String) -> Result<DiffJson, SceneJson> {
        let folder = "/var/www/games/".to_string() + &game_name + "/";
        let contents = Diff::read_file(folder.clone() + "DefaultScene.json");
        let diff_json: DefaultSceneJson = serde_json::de::from_str(&contents).unwrap();

        for diff in diff_json.diffs {
            if diff.hash == hash {
                return Ok(diff);
            }
        }
        let ret: SceneJson = serde_json::de::from_str(&Diff::read_file(folder + &diff_json.name)).unwrap();
        Err(ret)
    }

    fn read_file(filename: String) -> String {
        return std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    }
}