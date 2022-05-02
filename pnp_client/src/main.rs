pub mod api_json;
pub mod camera;
pub mod diff_json;
pub mod game_picker;
pub mod params;
pub mod scene;
pub mod scene_json;
#[cfg(test)]
mod tests;
pub mod token;
pub mod types;
pub mod ui;

use macroquad::prelude::*;
use scene_json::ClickAction;

use crate::game_picker::*;
use crate::params::*;
use crate::scene::*;
use crate::ui::*;

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
    loop {
        if let Some(action) = scene.click() {
            match action {
                ClickAction::SceneChange(scene) => {
                    next_scene = Some(scene);
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
