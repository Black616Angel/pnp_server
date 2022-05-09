#![allow(dead_code)]

use crate::fs::File;
use crate::json::scene_json::*;

#[macroquad::test]
async fn json_default_scene() {
    let contents = File::read_string("../files/games/testgame/DefaultScene.json").await;
    if let Ok(contents) = contents {
        let res: Result<DefaultSceneJson, serde_json::Error> = serde_json::from_str(&contents);
        if let Ok(ds) = res {
            assert_eq!(ds.name, "Test_Scene.json");
        } else {
            assert!(res.is_err());
        }
    } else {
        assert!(contents.is_err());
    }
}

#[macroquad::test]
async fn json_scene() {
    let contents: Result<String, macroquad::file::FileError> =
        File::read_string("../files/games/testgame/Test_Scene.json").await;
    if let Ok(contents) = contents {
        let res: Result<DefaultSceneJson, serde_json::Error> = serde_json::from_str(&contents);
        if let Ok(ds) = res {
            assert_eq!(ds.name, "Test_Scene.json");
        } else {
            assert!(res.is_err());
        }
    } else {
        assert!(contents.is_err());
    }
}
