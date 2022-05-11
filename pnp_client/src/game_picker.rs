use std::error::Error;

use crate::fs::File;
use crate::json::api_json::StringVecJson;
use crate::json::scene_json::*;
use crate::scene::*;

use macroquad::prelude::*;

pub struct GamePicker {
    pub root_folder: String,
}

impl GamePicker {
    pub fn new(folder: String) -> Self {
        Self {
            root_folder: folder,
        }
    }

    pub async fn get_scene(&self) -> Result<Scene, Box<dyn Error>> {
        let scene_json = self.build_scene_json().await?;

        let scene = Scene::new_from_json(self.root_folder.clone(), scene_json).await;

        return Ok(scene);
    }

    async fn build_scene_json(&self) -> Result<SceneJson, Box<dyn Error>> {
        let mut scene = SceneJson {
            name: "Game Picker".to_string(),
            height: 3,
            width: 8,
            square_size: 70,
            texture_background: None,
            tokens: Vec::new(),
        };
        let game_names = Self::get_game_folders(&("api/games/get_all")).await?;

        let mut row = 1;
        let mut col = 1;
        for game in game_names {
            info!("game: {}", game);
            let last_scene = self.get_last_scene_name(&game).await?;
            info!("last_scene: {}", last_scene);
            let token = SceneJsonToken {
                name: game.clone(),
                height: 2,
                width: 2,
                position_x: col,
                position_y: row,
                stats: None,
                description: Some("".to_string()),
                click_action: Some(ClickAction::SceneChange(game + "/" + &last_scene)),
                texture_path: "objects/defaults/game_texture.png".to_string(), //TODO: pictures for games
            };
            scene.tokens.push(token);
            if col >= 6 {
                col = 1;
                row += 3;
            } else {
                col += 2;
            }
        }
        scene.height = row + 2;
        return Ok(scene);
    }

    async fn get_last_scene_name(&self, game: &str) -> Result<String, Box<dyn Error>> {
        let contents =
            File::read_string(&(self.root_folder.clone() + "games/" + game + "/DefaultScene.json"))
                .await?;
        let json: DefaultSceneJson = serde_json::from_str(&contents)?;
        return Ok(json.name);
    }

    async fn get_game_folders(folder: &str) -> Result<Vec<String>, Box<dyn Error>> {
        info!("get_game_folders: {}", folder);
        let ret: Result<StringVecJson, serde_json::Error> =
            serde_json::de::from_str(&File::read_string(folder).await?);
        if ret.is_ok() {
            let ret: Vec<String> = ret.unwrap().values;
            return Ok(ret);
        } else {
            // must be error
            if let Some(err) = ret.err() {
                return Err(Box::new(err));
            } else {
                //cannot happen
                return Ok(Vec::new());
            }
        }
    }
}
