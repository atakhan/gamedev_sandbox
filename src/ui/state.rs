use bevy::ecs::system::Resource;

#[derive(Resource, Debug)]
pub struct UiVisibility {
    pub show_camera: bool,
    pub show_player: bool,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self {
            show_camera: false,
            show_player: false,
        }
    }
}
