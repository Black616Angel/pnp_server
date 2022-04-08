pub mod camera;
pub mod types;
pub mod token;
pub mod scene;
pub mod scene_json;
pub mod ui;
pub mod game_picker;

use macroquad::prelude::*;
use scene_json::ClickAction;

use crate::scene::*;
use crate::ui::*;
use crate::types::*;
use crate::game_picker::*;

#[macroquad::main("PnP")]
async fn main(){

    // Gamepicker as root of it all
    let gp = GamePicker::new("files".to_string());
    let mut scene = gp.get_scene().await.unwrap();
    info!("Started");

    let mut ui = UIList::new();

    let mut next_scene: Option<String> = None;
    loop {
        if let Some(action) =  scene.click() {
            match action {
                ClickAction::SceneChange(scene) => {
                    next_scene = Some(scene);
                },
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
                let mut split = next_scene_name.split("/");
                let folder = gp.root_folder.clone() + "/games/" + split.next().unwrap();
                let filename = folder.clone() + "/" + split.next().unwrap();
                info!("{}", filename);
                if let Ok(new) = Scene::new_from_file(filename, Some(folder)).await {
                    info!("{}", scene.name);
                    scene = new;
                };
            } else {

            }
        }
    }
}