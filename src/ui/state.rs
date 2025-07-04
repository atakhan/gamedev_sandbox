use bevy::ecs::{schedule::States, system::Resource};

#[derive(Resource, Debug)]
pub struct ScenePanelsVisibility {
    pub show_camera: bool,
    pub show_player: bool,
}

impl Default for ScenePanelsVisibility {
    fn default() -> Self {
        Self {
            show_camera: false,
            show_player: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum Screens {
    SceneManager,  // Меню выбора сцен
    SceneView,     // Просмотр/редактирование сцены
}
impl Default for Screens {
    fn default() -> Self {Self::SceneManager}
}

#[derive(Resource, Debug, Clone)]
pub struct ScreenState {
    pub current_screen: Screens,
}
impl Default for ScreenState {
    fn default() -> Self {
        Self {
            current_screen: Screens::default()
        }
    }
}

