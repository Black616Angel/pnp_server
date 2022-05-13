pub mod camera;
pub mod json;
pub mod game_picker;
pub mod params;
pub mod fs;
pub mod scene;
#[cfg(test)]
mod tests;
pub mod token;
pub mod types;
pub mod ui;
pub mod user;

use macroquad::prelude::*;
use json::scene_json::ClickAction;

use crate::game_picker::*;
use crate::params::*;
use crate::scene::*;
use crate::ui::*;
use crate::fs::File;

#[macroquad::main("PnP")]
async fn main() {
    // Gamepicker as root of it all
    info!("Started5");

    for param in PROGRAM_PARAMETERS.clone() {
        info!("{}: {}", param.key, param.value);
    }

    let gp = GamePicker::new("/".to_string());
    let mut scene = gp.get_scene().await.unwrap();
    let mut ui = UIList::new();

    let mut next_scene: Option<String> = None;
    let mut game_name = "None".to_string();
    loop {
        if let Some(action) = scene.click() {
            match action {
                ClickAction::SceneChange(scene) => {
                    next_scene = Some(scene);
                },
                ClickAction::Moved(diff) => {
                    match File::write(&format!("api/games/diff/{}", game_name.clone()), serde_json::json!(diff).to_string()).await {
                        Ok(_) => {},
                        Err(e) => {error!("{:?}", e);}
                    }
                }
                _ => {}
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            ui.click();
        }

        scene.draw();
        ui.draw();
        next_frame().await;
        if let Some(next_scene_name) = next_scene {
            next_scene = None;
            if scene.name == "Game Picker" {
                let split = next_scene_name.split("/").collect::<Vec<&str>>();
                if split.len() == 2 {
                    let folder = gp.root_folder.clone() + "games/" + split[0] + "/";
                    game_name = split[0].to_string();
                    let filename = folder.clone() + split[1];
                    // info!("filename: {}", filename);
                    if let Ok(new) = Scene::new_from_file(filename, Some(folder)).await {
                        info!("scene.name: {}", scene.name);
                        scene = new;
                    };
                } else {
                }
            } else {
            }
        }
    }
}
