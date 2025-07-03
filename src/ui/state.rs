use bevy::ecs::system::Resource;

#[derive(Resource, Debug)]
pub struct UiVisibility {
    pub show_camera: bool,
    pub show_player: bool,
    pub show_save_scene_dialog: bool,
    pub new_scene_name: String,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self {
            show_camera: false,
            show_player: false,
            show_save_scene_dialog: false,
            new_scene_name: String::from("Default scene"),
        }
    }
}
