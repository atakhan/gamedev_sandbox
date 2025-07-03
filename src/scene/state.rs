use std::fs;
use serde_json;
use crate::scene::data::SceneData;
use std::path::PathBuf;

pub struct SceneState {
    pub current_scene: Option<SceneData>,
    pub available_scenes: Vec<SceneData>,
    pub scenes_dir: PathBuf,
}

impl SceneState {
    pub fn new(scenes_dir: PathBuf) -> Self {
        Self {
            current_scene: None,
            available_scenes: Vec::new(),
            scenes_dir,
        }
    }

    pub fn load_scenes(&mut self) -> anyhow::Result<()> {
        self.available_scenes.clear();

        if !self.scenes_dir.exists() {
            fs::create_dir_all(&self.scenes_dir)?;
        }

        for entry in fs::read_dir(&self.scenes_dir)? {
            let path = entry?.path();
            if path.extension().map(|ext| ext == "json").unwrap_or(false) {
                let data = fs::read_to_string(&path)?;
                let scene: SceneData = serde_json::from_str(&data)?;
                self.available_scenes.push(scene);
            }
        }
        Ok(())
    }
}
